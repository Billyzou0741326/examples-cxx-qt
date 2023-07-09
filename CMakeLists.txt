cmake_minimum_required(VERSION 3.24)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug")
endif()

option(USE_QT5 "Use Qt5 even if Qt6 found" OFF)
if(WIN32 OR APPLE)
    option(VCPKG "Use vcpkg for dependencies" ON)
else()
    option(VCPKG "Use vcpkg for dependencies" OFF)
endif()

if(VCPKG)
    if(USE_QT5)
        set(VCPKG_MANIFEST_FEATURES "qt5")
    else()
        set(VCPKG_MANIFEST_FEATURES "qt6")
    endif()

    include(InitializeVcpkg)

    # These are required for binary caching to work reliably across machines.
    set(VCPKG_FEATURE_FLAGS "-compilertracking")
    set(VCPKG_INSTALL_OPTIONS "--x-abi-tools-use-exact-versions")

    if(NOT DEFINED ENV{VCPKG_BINARY_SOURCES})
        if(WIN32)
            set(COMMAND_PREFIX "")
            set(EXE_SUFFIX ".exe")
            set(SCRIPT_SUFFIX ".bat")
            set(DOTNET_RUNTIME "")
        else()
            set(COMMAND_PREFIX "./")
            set(EXE_SUFFIX "")
            set(SCRIPT_SUFFIX ".sh")
            set(DOTNET_RUNTIME "mono")
        endif()

        # vcpkg can download NuGet, so bootstrap vcpkg if the executable is not found.
        if(NOT EXISTS "${VCPKG_ROOT}/vcpkg${EXE_SUFFIX}")
        message(STATUS "Bootstrapping vcpkg")
        execute_process(
            COMMAND "${COMMAND_PREFIX}bootstrap-vcpkg${SCRIPT_SUFFIX}"
            WORKING_DIRECTORY ${VCPKG_ROOT}
        )
        endif()

        message(STATUS "Setting up vcpkg binary caching with read-only access to GitHub Packages NuGet source")

        execute_process(
            COMMAND "${COMMAND_PREFIX}vcpkg${EXE_SUFFIX}" fetch nuget
            WORKING_DIRECTORY ${VCPKG_ROOT}
            OUTPUT_VARIABLE NUGET_FETCH_OUTPUT
            OUTPUT_STRIP_TRAILING_WHITESPACE
        )
        string(REPLACE "\n" ";" NUGET_FETCH_OUTPUT "${NUGET_FETCH_OUTPUT}")
        list(POP_BACK NUGET_FETCH_OUTPUT NUGET_EXECUTABLE)

        # NuGet will fail with an error when trying to add a source with the same name
        # as one that already exists, so check that the NuGet source has not been added yet.
        execute_process(
            COMMAND ${DOTNET_RUNTIME} ${NUGET_EXECUTABLE} sources list
            OUTPUT_VARIABLE NUGET_SOURCES_LIST
        )
        string(FIND "${NUGET_SOURCES_LIST}" "cxx-qt-github-packages" SEARCH_RESULT)
        if(SEARCH_RESULT EQUAL -1)
        # GitHub will deactivate a personal access token that gets committed to the repository.
        # Hack around this by splitting up the PAT.
        # This is safe because this PAT only has read:packages permission.
        set(GITHUB_PAT_READ_PACKAGES_SUFFIX HvVJ7NF8sArqcyBnF45RXOgAT0Q1uL42CZkO)
        execute_process(
            COMMAND ${DOTNET_RUNTIME} ${NUGET_EXECUTABLE} sources add
            -name cxx-qt-github-packages
            -source https://nuget.pkg.github.com/KDAB/index.json
            -username KDAB
            -password ghp_${GITHUB_PAT_READ_PACKAGES_SUFFIX}
        )
        endif()

        set(ENV{VCPKG_BINARY_SOURCES} "clear;default,readwrite;nuget,cxx-qt-github-packages,read;")
    endif()
else()
    message(STATUS "Using dependencies from system without vcpkg")
endif()

project(examples_cxx_qt)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_compile_definitions(
    QT_NO_CAST_FROM_ASCII
    QT_NO_CAST_TO_ASCII
    QT_NO_CAST_FROM_BYTEARRAY
    QT_NO_URL_CAST_FROM_STRING
    QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
    QT_NO_FOREACH
    QT_NO_JAVA_STYLE_ITERATORS
    QT_NO_KEYWORDS
    QT_USE_QSTRINGBUILDER
)

# QMAKE environment variable is needed by qt-build-utils to ensure that Cargo
# uses the same installation of Qt as CMake does.
if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
set(CARGO_ENV "QMAKE=set:${QMAKE}")
set(RUNTIME_ENV "")

# On windows, Qt dll needs to be in the PATH for the tests to run
if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_BINS
        OUTPUT_VARIABLE QT_INSTALL_BINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_PLUGINS
        OUTPUT_VARIABLE QT_INSTALL_PLUGINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_QML
        OUTPUT_VARIABLE QT_INSTALL_QML
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    list(
        APPEND
        RUNTIME_ENV
        "PATH=path_list_append:${QT_INSTALL_BINS}"
        "QT_PLUGIN_PATH=path_list_append:${QT_INSTALL_PLUGINS}"
        "QML_IMPORT_PATH=path_list_append:${QT_INSTALL_QML}"
        "QML2_IMPORT_PATH=path_list_append:${QT_INSTALL_QML}"
    )
    list(APPEND CARGO_ENV ${RUNTIME_ENV})
endif()

# Same logic as in Corrosion.cmake
if(CMAKE_VS_PLATFORM_NAME)
    set(BUILD_DIR "${CMAKE_VS_PLATFORM_NAME}/$<CONFIG>")
elseif(CMAKE_CONFIGURATION_TYPES)
    set(BUILD_DIR "$<CONFIG>")
else()
    set(BUILD_DIR .)
endif()

# Set the target dir to the same that Corrosion uses to reuse build artifacts
# from the main build.
set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/${BUILD_DIR}/cargo/build")

add_subdirectory(cargo_without_cmake)
