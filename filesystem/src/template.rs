pub enum Lang {
    C,
    CPP,
}

pub struct Template(Lang);

impl Template {
    pub fn new_bin_template(language: Lang) -> Self {
        let c_bin_template = vec![
            vec!["src", "include", "build"],
            vec!["src/main.c", "CMakeLists.txt", "include/main.h"],
        ];
        let cpp_bin_template = vec![
            vec!["src", "include", "build"],
            vec!["src/main.cpp", "CMakeLists.txt", "inlcude/main.h"],
        ];

        match language {
            Lang::C => {}
            Lang::CPP => {}
        }

        return Self(language);
    }

    pub fn new_lib_template() -> Self {
        Self
    }

    pub fn new_cmake_template() -> Self {
        Self
    }
}
