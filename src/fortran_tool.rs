/// Configuration used to represent an invocation of a Fortran Compiler.
///
/// This can be used to figure out what compiler is in use, what the arguments
/// to it are, and what the environment variables look like for the compiler.
/// This can be used to further configure other build systems (e.g. forward
/// along FC and/or FFLAGS) or the `to_command` method can be used to run the
/// compiler itself.
#[derive(Clone, Debug)]
pub struct Tool {

}

/// Represents the family of tools this tool belongs to.
///
/// Each family of tools differs in how and what arguments they accept.
///
/// Detection of a family is done on best-effort basis and may not accurately reflect the tool.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolFamily {
    /// Tool is GNU Fortran Compiler-like
    GFortran,
    /// Tool is Intel IFX Fortran Compiler-like
    IntelIFX,
    /// Tool is Intel Classic Fortran Compiler-like
    IntelClassic,
    /// Tool is Flang Fortran Compiler-like
    Flang,
}
