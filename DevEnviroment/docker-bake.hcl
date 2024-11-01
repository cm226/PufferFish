variable NASM_VERSION { default = "2.16" }
variable RUST_VERSION { default = "1.82" }

target "default" {
  context = "./"
  dockerfile = "./Dockerfile"
  args = {
    NASM_VERSION = "${NASM_VERSION}"
    RUST_VERSION = "${RUST_VERSION}"
  }
  tags = ["cm226/pufferfish:${RUST_VERSION}-${NASM_VERSION}"]
}
