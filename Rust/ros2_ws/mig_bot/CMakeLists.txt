cmake_minimum_required(VERSION 3.5)
project(mig_bot)

find_package(ament_cmake REQUIRED)
find_package(r2r REQUIRED)

ament_package()

add_custom_target(
  mig_bot ALL
  COMMAND cargo build --release
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

install(
  DIRECTORY target/release/
  DESTINATION lib/${PROJECT_NAME}
  FILES_MATCHING PATTERN "*"
)
