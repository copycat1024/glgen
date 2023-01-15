pub enum TypeMod {
    None,
    Ptr,
    PtrPtr,
    ConstPtr,
    ConstPtrPtr,
}

pub enum TypeData {
    Char,
    DebugProc,
    Double,
    Float,
    Int,
    Int64,
    IntSize,
    Short,
    Uchar,
    Uint,
    Uint64,
    Ushort,
    Void,
}

impl TypeData {
    pub fn new(id: &str) -> Self {
        match id {
            "GLDEBUGPROC" => Self::DebugProc,
            "GLbitfield" => Self::Uint,
            "GLboolean" => Self::Uchar,
            "GLbyte" => Self::Char,
            "GLchar" => Self::Char,
            "GLclampx" => Self::Int,
            "GLdouble" => Self::Double,
            "GLenum" => Self::Uint,
            "GLfixed" => Self::Int,
            "GLfloat" => Self::Float,
            "GLint" => Self::Int,
            "GLint64" => Self::Int64,
            "GLintptr" => Self::IntSize,
            "GLshort" => Self::Short,
            "GLsizei" => Self::Int,
            "GLsizeiptr" => Self::IntSize,
            "GLubyte" => Self::Uchar,
            "GLuint" => Self::Uint,
            "GLuint64" => Self::Uint64,
            "GLushort" => Self::Ushort,
            "void" => Self::Void,
            id => unreachable!("unknown id {}", id),
        }
    }
}

pub struct Type {
    pub md: TypeMod,
    pub id: TypeData,
}

impl Type {
    pub fn new(name: &str) -> Self {
        let name = if name == "GLsync" { "void *" } else { name };
        let tokens: Vec<&str> = name.split(' ').collect();
        let (md, id) = match tokens.len() {
            1 => (TypeMod::None, tokens[0]),
            2 => match tokens[1] {
                "*" => (TypeMod::Ptr, tokens[0]),
                "**" => (TypeMod::PtrPtr, tokens[0]),
                _ => unreachable!(),
            },
            3 => match tokens[2] {
                "*" => (TypeMod::ConstPtr, tokens[1]),
                "*const*" => (TypeMod::ConstPtrPtr, tokens[1]),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        Self {
            md,
            id: TypeData::new(id),
        }
    }

    pub fn c(&self) -> String {
        format!("{}{}", self.mod_(), self.match_c())
    }

    pub fn rust(&self) -> String {
        format!("{}{}", self.mod_(), self.match_rust())
    }

    pub fn not_void(&self) -> bool {
        !matches!((&self.md, &self.id), (TypeMod::None, TypeData::Void))
    }

    fn mod_(&self) -> &str {
        match self.md {
            TypeMod::None => "",
            TypeMod::ConstPtr => "*const ",
            TypeMod::ConstPtrPtr => "*const *const ",
            TypeMod::Ptr => "*mut ",
            TypeMod::PtrPtr => "*mut *mut ",
        }
    }

    fn match_c(&self) -> &str {
        match self.id {
            TypeData::Char => "c_char",
            TypeData::DebugProc => "DebugProc",
            TypeData::Double => "c_double",
            TypeData::Float => "c_float",
            TypeData::Int => "c_int",
            TypeData::Int64 => "i64",
            TypeData::IntSize => "isize",
            TypeData::Short => "c_short",
            TypeData::Uchar => "c_uchar",
            TypeData::Uint => "c_uint",
            TypeData::Uint64 => "u64",
            TypeData::Ushort => "c_ushort",
            TypeData::Void => "c_void",
        }
    }

    fn match_rust(&self) -> &str {
        match self.id {
            TypeData::Char => "i8",
            TypeData::DebugProc => "DebugProc",
            TypeData::Double => "f64",
            TypeData::Float => "f32",
            TypeData::Int => "i32",
            TypeData::Int64 => "i64",
            TypeData::IntSize => "isize",
            TypeData::Short => "i16",
            TypeData::Uchar => "u8",
            TypeData::Uint => "u32",
            TypeData::Uint64 => "u64",
            TypeData::Ushort => "u16",
            TypeData::Void => "c_void",
        }
    }
}
