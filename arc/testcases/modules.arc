// low priority now to implement this in our compiler

mod module_name {
    fx a_function() {
        let a = 1;
    }

    pub fx b_function() ~ i32 {
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
