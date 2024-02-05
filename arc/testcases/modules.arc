mod module_name {
    fx a_function() {
        statement1;
    }

    pub fx b_function() ~ u32 {
        return 11;
    }

    mod child_module {
        import super::*;
        fx yay() {
          let sqrt_2 = that_module::sqrt(4);
          return sqrt_2;
        }
    }
}
