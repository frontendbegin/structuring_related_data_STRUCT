use std::f64;
use std::fmt;
use std::num;

struct Matrix(f32, f32, f32, f32);
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}, {}]\n[{}, {}]", self.0, self.1, self.2, self.3)
        }
    }
    impl Matrix {
        fn transpose(&self) -> Matrix{
            let plac1 = self.1;
            let plac2 = self.2;
           
            Matrix(self.0, plac2, plac1, self.3)
        }
    }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// struct Color(i32, i32, i32);
// struct Pointer(i32, i32, i32);

// #[derive(Debug)]
// struct Rectangle {
//     height: u32,
//     width: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {
//     fn non_zero_width(&self) -> bool {
//         self.width > 0
//     }
// }
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }
// impl Rectangle {
//     fn are_equal(&self) -> bool{
//         self.width == self.height
//     }
// }
// impl Rectangle{
//     fn diagonal(&self) -> f64{
//         let width_64 = self.width as f64;
//         let height_64 = self.height as f64;
//         let add_squares = squares_f64(width_64) + squares_f64(height_64);
//         add_squares.sqrt()

//     }
// }
fn main() {
    //     let username = String::from("someusername",);
    //     let email = String::from("someone@example.com");
    //  let user1 =build_user(username, email);

    // let  user2 = User {
    //     active: true,
    //     username: String::from("someusername"),
    //     email: user1.email,
    //     sign_in_count: 1,
    // };
    // let user3 = User{
    //     active: user2.active,
    //     username: user1.username,
    //     email: String::from ("another@example.com"),
    //     sign_in_count:  user2.sign_in_count,
    // };
    // let black = Color(0,0,0);
    // let center = Pointer(30,0,0);
    // let x = center.1;
    // println!("{x}");
    // // let user4 = User {
    // //     email: String::from("nextexample@example.com"),
    // //     ..user2
    // // }moves the value of user2.email cause it is a string
    // let user5 = User{
    //     username: user2.username,
    //     email: String::from ("another@example.com"),
    //     ..user2
    // };
    //     let rectangle1 = Rectangle {
    //         width: 30,
    //         height: 50,
    //     };

    //     // println!("The units of the rectangle is {rectangle1:#?}");
    //     println!("The area of the rectangle is {}", rectangle1.area());
    //     if rectangle1.non_zero_width() {
    //         println!(
    //             "This rectngle has a non zero width, its width is {}",
    //             rectangle1.width
    //         )
    //     };
    //     let rect1 = Rectangle {
    //         width: 30,
    //         height: 50,
    //     };
    //     let rect2 = Rectangle {
    //         width: 10,
    //         height: 40,
    //     };
    //     let rect3 = Rectangle {
    //         width: 60,
    //         height: 45,
    //     };
    //     let rect4 = Rectangle::square(40);
    //     println!("Are these squares");
    //     println!("Is rect1 a square {}", rect1.are_equal());
    //     println!("Is rect2 a square {}", rect2.are_equal());
    //     println!("Is rect3 a square {}", rect3.are_equal());
    //     println!("Is rect4 a square {}", rect4.are_equal());
    //     println!("The diagonal of rect 2 is {}", rect2.diagonal());

    //     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    //     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let matr1 = Matrix(1.1, 1.2, 1.3, 1.4);
   let matr2 =  matr1.transpose();
   println!("{matr1}");
    println!("{matr2}");

}

// fn build_user(username: String, email: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
// fn squares(x:u32) -> u32{
//     x*x
// }
// fn squares_f64(x:f64) -> f64{
//     x*x
// }
