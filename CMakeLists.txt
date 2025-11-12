cmake_minimum_required(VERSION 3.16)

project(OompaSale)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Core Quick)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

qt6_add_executable(${PROJECT_NAME}
    src/main.cpp
    qml.qrc
)

qt6_add_qml_module(${PROJECT_NAME}
    URI OompaSale
    VERSION 1.0
    QML_FILES
        qml/main.qml
)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt6::Core Qt6::Quick)
