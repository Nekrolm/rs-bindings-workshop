project(ClibDepEgs)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(clib_dep_egs STATIC
    clib_impl.cpp
)

install(TARGETS clib_dep_egs)