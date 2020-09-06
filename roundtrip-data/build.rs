use cycloneddscodegen as codegen;

fn main() {

  let idls = vec!["idl/Roundtrip.idl"];
  codegen::generate_and_compile_datatypes(idls);

}
