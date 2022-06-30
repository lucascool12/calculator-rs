#[allow(dead_code)]
pub mod model{
    use std::{
        boxed::Box,
        fmt,
        ptr::NonNull,
        time    
    };

    #[derive(Clone)]
    pub enum Value{
        Value(f64),
        Operator(Operator)
    }

    impl fmt::Display for Value{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            match self{
                Self::Value(val) => write!(f,"{}",val),
                Self::Operator(op) => {
                    write!(f,"{}",op)
                }
            }
        }
    }

    impl Value{
        pub fn new_op(op:Operator) -> Value{
            Value::Operator(op)
        }

        pub fn new_f64(val:f64) -> Value{
            Value::Value(val)
        }
    }
    
    #[derive(Clone)]
    pub enum Operator{
        Plus,
        Minus,
        Mult,
        Div
    }

    impl Operator{
        pub fn evaluate(&self, left:f64, right:f64) -> f64{
            match self{
                Operator::Plus => {
                    left + right
                },
                Operator::Minus => {
                    left - right
                },
                Operator::Mult => {
                    left*right
                },
                Operator::Div => {
                    left/right
                }
            }
        }
    }
    
    impl fmt::Display for Operator{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            match self{
                Operator::Plus => write!(f,"+"),
                Operator::Minus => write!(f,"-"),
                Operator::Mult => write!(f,"*"),
                Operator::Div => write!(f,"/")
            }
        }
    }

    pub struct Tree<T>{
        head: Option<NonNull<Node<T>>>,
        current: Option<NonNull<Node<T>>>
    }
    

    enum Dir{
        Right,
        Left,
        Up
    }

    #[derive(Debug)]
    pub enum TreeError{
        DeadEnd,
        OpOnNone
    }

    pub enum EvalError{
        BadTree,
        UnexpectedOp
    }

    impl fmt::Display for EvalError{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            match self{
                EvalError::BadTree => write!(f, "BadTree"),
                EvalError::UnexpectedOp => write!(f, "UnexpectedOp")
            }
        }
    }

    impl fmt::Display for TreeError{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            match self{
                TreeError::DeadEnd => write!(f, "DeadEnd"),
                TreeError::OpOnNone => write!(f, "OpOnNone")
            }
        }
    }

    impl<T:fmt::Display> Tree<T>{
        pub fn display_tree(&mut self){
            match self.head{
                Some(n) => {
                    unsafe{
                        (*n.as_ptr()).display_rec(0);
                    }
                },
                None => ()
            }
        }
    }

    impl<T> Tree<T>{
        fn new() -> Self{
            Tree{
                head: None,
                current: None
            }
        }

        fn push(&mut self, new_head: T, dir:Dir) -> Result<(),TreeError>{
            match self.current{
                Some(old) =>{
                    let up_opt;
                    unsafe{
                        up_opt = (*old.as_ptr()).up;
                    }
                    let mut new = Box::new(Node::new(new_head));
                    match dir{
                        Dir::Right => new.set_right_ptr(Some(old)),
                        Dir::Left => new.set_left_ptr(Some(old)),
                        Dir::Up => ()
                    }
                    let ptr:Option<NonNull<Node<T>>> = Some(Box::leak(new).into());
                    match up_opt{
                        Some(up) => {
                            unsafe{
                                if old.eq(&up){
                                    (*up.as_ptr()).set_left_ptr(ptr);
                                }else{
                                    (*up.as_ptr()).set_right_ptr(ptr);
                                }
                            }
                        },
                        None => ()
                    }
                    unsafe{
                        (*old.as_ptr()).up = ptr;
                    }
                    Ok(())
                }
                None => Err(TreeError::OpOnNone)
            }
        }

        pub fn push_left(&mut self, new_head: T) -> Result<(),TreeError>{
            self.push(new_head, Dir::Left)
        }

        pub fn push_right(&mut self, new_head: T) -> Result<(),TreeError>{
            self.push(new_head, Dir::Right)
        }

        fn set_current(&mut self, n: T){
            match self.current{
                Some(node) => {
                    unsafe{
                        (*node.as_ptr()).value = Some(n);
                    }
                },
                None => {
                    self.current = Some(Box::leak(Box::new(Node::new(n))).into());
                    if self.head == None{
                        self.head = self.current;
                    }
                }
            }
        }

        fn get_current(&self) -> Result<&Option<T>,TreeError>{
            unsafe{
                match self.current{
                    Some(node) => {
                        Ok(&node.as_ref().value)
                    },
                    None => Err(TreeError::OpOnNone)
                }
            }
        }

        fn get_left(&self) -> Result<&Option<T>,TreeError>{
            unsafe{
                if let Some(cur) = self.current{
                    if let Some(left) = (*cur.as_ptr()).left{
                        Ok(&left.as_ref().value)
                    }else{
                        Err(TreeError::OpOnNone)
                    }
                }else{
                    Err(TreeError::OpOnNone)
                }
            }
        }

        fn get_right(&self) -> Result<&Option<T>,TreeError>{
            unsafe{
                if let Some(cur) = self.current{
                    if let Some(right) = (*cur.as_ptr()).right{
                        Ok(&right.as_ref().value)
                    }else{
                        Err(TreeError::OpOnNone)
                    }
                }else{
                    Err(TreeError::OpOnNone)
                }
            }
        }

        fn set_child(&mut self, n: T, dir:Dir) -> Result<(), TreeError>{
            match self.current{
                Some(node) => {
                    
                    unsafe{
                        match dir{
                            Dir::Left => {
                                
                                match (*node.as_ptr()).left{
                                    Some(s) => {
                                        (*s.as_ptr()).value = Some(n);
                                    },
                                    None => {
                                        let new = Box::new(Node::new(n));
                                        (*node.as_ptr()).set_left(Some(new))
                                    }
                                }
                            },
                            Dir::Right => {
                                match (*node.as_ptr()).right{
                                    Some(s) => {
                                        (*s.as_ptr()).value = Some(n);
                                    },
                                    None => {
                                        let new = Box::new(Node::new(n));
                                        (*node.as_ptr()).set_right(Some(new))
                                    }
                                }
                            },
                            Dir::Up => ()
                        }
                    }
                    Ok(())
                },
                None => {
                    Err(TreeError::OpOnNone)
                }
            }
        }

        pub fn set_child_left(&mut self, new: T) -> Result<(), TreeError>{
            self.set_child(new, Dir::Left)
        }

        pub fn set_child_right(&mut self, new: T) -> Result<(), TreeError>{
            self.set_child(new, Dir::Right)
        }

        pub fn select_root(&mut self){
            self.current = self.head;
        }

        fn go(&mut self, dir:Dir) -> Result<(), TreeError>{
            match self.current{
                Some(node) =>{
                    unsafe{
                        match dir{
                            Dir::Left => {
                                match (*node.as_ptr()).left{
                                    Some(node) => self.current = Some(node),
                                    None => return Err(TreeError::DeadEnd)
                                }
                                Ok(())
                            },
                            Dir::Right => {
                                match (*node.as_ptr()).right{
                                    Some(node) => self.current = Some(node),
                                    None => return Err(TreeError::DeadEnd)
                                }
                                Ok(())
                            },
                            Dir::Up => {
                                match (*node.as_ptr()).up{
                                    Some(node) => self.current = Some(node),
                                    None => return Err(TreeError::DeadEnd)
                                }
                                Ok(())
                            }
                        }
                    }
                },
                None => Err(TreeError::OpOnNone)
            }
        }

        pub fn go_left(&mut self) -> Result<(), TreeError>{
            self.go(Dir::Left)
        }

        pub fn go_right(&mut self) -> Result<(), TreeError>{
            self.go(Dir::Right)
        }

        pub fn go_up(&mut self) -> Result<(), TreeError>{
            self.go(Dir::Up)
        }
    }

    impl Tree<Value>{
        pub fn evaluate_it2(&mut self) -> Result<f64, EvalError>{
            let mut ptr_stack: Vec<NonNull<Node<Value>>> = Vec::new();
            // let mut val_buffer: Option<f64> = None;
            let mut buf_len = 0;
            let mut val_buffer = [0f64;2];
            self.select_root();
            // let mut left_buffer = 0f64;
            // let mut right_buffer = 0f64;
            loop{
                // println!("len: {} ",buf_len);
                
                if let Ok(Some(val_ref)) = self.get_current(){
                    let val = val_ref.clone();
                    match val{
                        Value::Value(num) => return Ok(num),
                        Value::Operator(op) => {
                            if let (Ok(Some(left_val_ref)), Ok(Some(right_val_ref)))
                                     = (self.get_left(), self.get_right()){
                                // println!("  {}",right_val_ref);
                                // println!("{}",op);
                                // println!("  {}",left_val_ref);
                                let left_val = left_val_ref.clone();
                                let right_val = right_val_ref.clone();
                                match (left_val, right_val){
                                    (Value::Value(left_f64), Value::Value(right_f64)) => {
                                        // println!("{} {}",left_f64, right_f64);
                                        val_buffer[buf_len] = op.evaluate(left_f64, right_f64);
                                        buf_len += 1;
                                        // for i in val_buffer{
                                        //     print!("{} ",i);
                                        // }
                                        // println!("");
                                        match ptr_stack.pop(){
                                            Some(prev) => self.current = Some(prev),
                                            None => break
                                        };
                                    },
                                    (Value::Operator(_),Value::Value(right_f64)) => {
                                        // println!("{}", right_f64);
                                        if buf_len > 0 {
                                            val_buffer[buf_len - 1] = op.evaluate(val_buffer[buf_len - 1], right_f64);
                                            // for i in val_buffer{
                                            //     print!("{} ",i);
                                            // }
                                            // println!("");
                                            self.current = ptr_stack.pop();
                                        }else{
                                            ptr_stack.push(self.current.unwrap());
                                            match self.go_left(){Ok(_) => (), Err(_) => return Err(EvalError::BadTree)};
                                        }
                                    },
                                    (Value::Value(left_f64),Value::Operator(_)) => {
                                        
                                        if buf_len > 0 {
                                            // println!("v op {} {}", left_f64, val_buffer[buf_len]);
                                            val_buffer[buf_len - 1] = op.evaluate(left_f64, val_buffer[buf_len - 1]);
                                            
                                            // for i in val_buffer{
                                            //     print!("{} ",i);
                                            // }
                                            // println!("");
                                            match ptr_stack.pop(){
                                                Some(prev) => self.current = Some(prev),
                                                None => break
                                            };
                                        }else{
                                            ptr_stack.push(self.current.unwrap());
                                            match self.go_right(){Ok(_) => (), Err(_) => return Err(EvalError::BadTree)};
                                        }
                                    }
                                    (Value::Operator(_),Value::Operator(_)) => {
                                        if buf_len > 1 {
                                            let dir: usize;
                                            if self.current.eq(unsafe{&(*self.current.unwrap().as_ptr()).left}){
                                                dir = 0;
                                            }else{
                                                dir = 1;
                                            }
                                            val_buffer[dir] = op.evaluate(val_buffer[0], val_buffer[1]);
                                            // for i in val_buffer{
                                            //     print!("{} ",i);
                                            // }
                                            // println!("");
                                            match ptr_stack.pop(){
                                                Some(prev) => self.current = Some(prev),
                                                None => break
                                            };
                                        }else if buf_len > 0{
                                            ptr_stack.push(self.current.unwrap());
                                            match self.go_right(){Ok(_) => (), Err(_) => return Err(EvalError::BadTree)};
                                        }else{
                                            ptr_stack.push(self.current.unwrap());
                                            match self.go_left(){Ok(_) => (), Err(_) => return Err(EvalError::BadTree)};
                                        }
                                    }
                                }
                            }else{
                                return Err(EvalError::BadTree);
                            }
                        }
                    }
                }else{
                    return Err(EvalError::BadTree);
                }
            }
            Ok(val_buffer[0])
        }

        pub fn evaluate_it1(&mut self) -> Result<f64, EvalError>{
            self.select_root();
            let mut val_stack: Vec<Value> = Vec::new();
            let mut ptr_stack: Vec<NonNull<Node<Value>>> = Vec::new();

            loop{
                if let Ok(Some(v)) = self.get_current(){
                    val_stack.push(v.clone());
                }else{
                    return Err(EvalError::BadTree);
                }
            
                if let Some(node) = self.current{
                    
                    match unsafe{(*node.as_ptr()).left}{
                        Some(_) => {
                            ptr_stack.push(self.current.unwrap());
                            match self.go_left(){Ok(_)=>(),Err(_)=> return Err(EvalError::BadTree)};

                        },
                        None => {
                            self.current = match ptr_stack.pop(){
                                Some(ptr) => Some(ptr),
                                None => {
                                    loop{
                                        let val_len = val_stack.len();
                                        if val_len >= 3{
                                            let first = val_stack.get(val_len - 3).unwrap().clone();
                                            let second = val_stack.get(val_len - 2).unwrap().clone();
                                            let third = val_stack.get(val_len - 1).unwrap().clone();

                                            if let (Value::Operator(op), Value::Value(left), Value::Value(right))
                                                = (first, second, third){
                                                val_stack.remove(val_len - 1);
                                                val_stack.remove(val_len - 2);
                                                val_stack.remove(val_len - 3);
                                                val_stack.push(Value::Value(op.evaluate(left.clone(), right.clone())));
                                            }else{
                                                break;
                                            }
                                        }else{
                                            break;
                                        }
                                    }
                                    if val_stack.len() == 1 {
                                        if let Some(Value::Value(f)) = val_stack.pop(){
                                            return Ok(f);
                                        }else{
                                            return Err(EvalError::BadTree);
                                        }
                                    }else{
                                        return Err(EvalError::BadTree);
                                    }
                                }
                            };
                            match self.go_right(){Ok(_)=>(),Err(_)=> return Err(EvalError::BadTree)};
                        }
                    }
                }
                loop{
                    let val_len = val_stack.len();
                    // println!("check:");
                    if val_len >= 3{
                        let first = val_stack.get(val_len - 3).unwrap().clone();
                        let second = val_stack.get(val_len - 2).unwrap().clone();
                        let third = val_stack.get(val_len - 1).unwrap().clone();
                        // println!("check: {} {} {}", first, second, third);
                        if let (Value::Operator(op), Value::Value(left), Value::Value(right))
                            = (first, second, third){
                            val_stack.remove(val_len - 1);
                            val_stack.remove(val_len - 2);
                            val_stack.remove(val_len - 3);
                            val_stack.push(Value::Value(op.evaluate(left.clone(), right.clone())));
                        }else{
                            break;
                        }
                    }else{
                        break;
                    }
                }
            }
        }

        pub fn rec_evaluate(&mut self) -> Result<f64, EvalError>{
            
            match self.head{
                Some(node) => {
                    unsafe{
                        (*node.as_ptr()).rec_evaluate()
                    }
                },
                None => Err(EvalError::BadTree)
            }
            
        }
    }

    pub struct Node<T>{
        up: Option<NonNull<Node<T>>>,
        left: Option<NonNull<Node<T>>>,
        pub value: Option<T>,
        right: Option<NonNull<Node<T>>>
    }

    impl Node<Value>{
        fn rec_evaluate(&self) -> Result<f64, EvalError>{
            match &self.value{
                Some(val) => {
                    match val{
                        Value::Value(num) => Ok(num.clone()),
                        Value::Operator(op) => {
                            unsafe{
                                let left = match self.left{
                                    Some(node) => {
                                        match (*node.as_ptr()).rec_evaluate(){
                                            Ok(s) => s,
                                            Err(e) => return Err(e)
                                        }
                                    },
                                    None => 0f64,
                                };

                                let right = match self.right{
                                    Some(node) => {
                                        match (*node.as_ptr()).rec_evaluate(){
                                            Ok(s) => s,
                                            Err(e) => return Err(e)
                                        }
                                    },
                                    None => 0f64,
                                };

                                Ok(op.evaluate(left, right))
                            }
                        }
                    }
                },
                None => return Err(EvalError::BadTree)
            }
        }
    }


    impl<T> Node<T>{
        fn new(val: T) -> Self{
            Node{
                up: None,
                left: None,
                value: Some(val),
                right: None
            }
        }

        pub fn get_left(&self) -> Option<NonNull<Self>>{
            match self.left{
                Some(n) => {
                    unsafe{(*n.as_ptr()).left}
                }
                None => None
            }
        }

        pub fn set_right(&mut self, node_right: Option<Box<Self>>){
            match node_right{
                Some(mut node) => {
                    node.up = NonNull::new(self);
                    self.right = Some(Box::leak(node).into())
                },
                None => self.right = None
            }
        }

        pub fn set_right_ptr(&mut self, right_ptr: Option<NonNull<Self>>){
            match right_ptr{
                Some(node) => {
                    unsafe{
                        (*node.as_ptr()).up = NonNull::new(self);
                    }
                    self.right = right_ptr
                },
                None => self.right = None
            }
        }

        pub fn set_left(&mut self, node_left: Option<Box<Self>>){
            match node_left{
                Some(mut node) => {
                    node.up = NonNull::new(self);
                    self.left = Some(Box::leak(node).into())
                },
                None => self.left = None
            }
        }

        pub fn set_left_ptr(&mut self, left_ptr: Option<NonNull<Self>>){
            match left_ptr{
                Some(node) => {
                    unsafe{
                        (*node.as_ptr()).up = NonNull::new(self);
                    }
                    self.left = left_ptr
                },
                None => self.left = None
            }
        }
    }

    impl<T:fmt::Display> Node<T>{
        pub fn display_rec(&self, depth: u32){
            match self.right{
                Some(n) => {
                    unsafe{
                        (*n.as_ptr()).display_rec(depth + 1);
                    }
                },
                None => ()
            }

            for _ in 0..(depth){   
                print!("   ");
            }
            match &self.value{
                Some(valu) => println!("{}", valu),
                None => println!("None\n")
               
            }

            match self.left{
                Some(n) => {
                    unsafe{
                        (*n.as_ptr()).display_rec(depth + 1);
                    }
                },
                None => ()
            }
        }
    }
    

    pub fn parse_to_tree<'a>(src: &str) -> Result<Node<Value>, &str>{

        let mut temp = String::new();
        


        for (_i,c) in src.char_indices(){
            // println!("{} {}",i, c);
            match c{
                '0' => {
                    temp.push_str("0");
                    continue;
                },
                '1' => {
                    temp.push_str("1");
                    continue;
                },
                '2' => {
                    temp.push_str("2");
                    continue;
                },
                '3' => {
                    temp.push_str("3");
                    continue;
                },
                '4' => {
                    temp.push_str("4");
                    continue;
                },
                '5' => {
                    temp.push_str("5");
                    continue;
                },
                '6' => {
                    temp.push_str("6");
                    continue;
                },
                '7' => {
                    temp.push_str("7");
                    continue;
                },
                '8' => {
                    temp.push_str("8");
                    continue;
                },
                '9' => {
                    temp.push_str("9");
                    continue;
                },
                '.' => {
                    temp.push_str(".");
                    continue;
                },
                _ => ()
            };
            if temp.eq(".") {
                return Err("Bad float");
            }
            
            println!("{}",&temp);

            temp.truncate(0);
            
            match c{
                '+' => {
                    
                },
                '-' => println!("{}",c),
                '*' => println!("{}",c),
                '/' => println!("{}",c),
                _ => {return Err("Wrong input")}
            };

            
        }
        println!("{}",&temp);


        Ok(Node{
            up: None,
            left: None,
            value: Some(Value::Value(0f64)),
            right: None
        })
    }

    pub fn test() -> Result<(), TreeError>{
        let mut tree:Tree<Value> = Tree::new();
        tree.set_current(Value::Operator(Operator::Mult));
        tree.set_child_right(Value::Operator(Operator::Mult))?;
        tree.set_child_left(Value::Value(2f64))?;
        for _ in 0..5{
            tree.go_right()?;
        tree.set_child_left(Value::Value(2f64))?;
        tree.set_child_right(Value::Operator(Operator::Mult))?;
        }
        tree.set_child_right(Value::Value(2f64))?;
        
        tree.display_tree();
        let start = time::Instant::now();
        match tree.rec_evaluate(){
            Ok(num) => println!("{}, elapsed time in ms {}",num, start.elapsed().as_micros()),
            Err(e) => println!("{}",e)
        }
        
        let start = time::Instant::now();
        match tree.evaluate_it1(){
            Ok(num) => println!("{}, elapsed time in ms {}",num, start.elapsed().as_micros()),
            Err(e) => println!("{}",e)
        }

        let start = time::Instant::now();
        match tree.evaluate_it2(){
            Ok(num) => println!("{}, elapsed time in ms {}",num, start.elapsed().as_micros()),
            Err(e) => println!("{}",e)
        }
        
        Ok(())
    }
}