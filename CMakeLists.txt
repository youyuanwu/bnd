cmake_minimum_required(VERSION 3.20)
project(bnd NONE) # no native compiler needed

find_program(CARGO cargo REQUIRED)

# ---------- individual generators (order matters) ----------

add_custom_target(generate-linux
    COMMAND ${CARGO} run -p bnd-linux-gen
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Generating bnd-linux bindings"
    VERBATIM
)

add_custom_target(generate-openssl
    COMMAND ${CARGO} run -p bnd-openssl-gen
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Generating bnd-openssl bindings"
    VERBATIM
)

# bnd-openssl-gen requires bnd-linux.winmd produced by bnd-linux-gen
add_dependencies(generate-openssl generate-linux)

# ---------- umbrella target ----------

add_custom_target(generate ALL DEPENDS generate-linux generate-openssl)
