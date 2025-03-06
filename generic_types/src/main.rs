struct Point <T> {
    x : T , 
    y : T 
}

impl <T> Point<T> {
    fn x (&self)-> &T {
        return &self.x ; 
    }
}

struct Notes <T,U> {
    s5 : U , 
    s6 : T  
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point1 = Point {
        x : 3 , 
        y : 5 
    };
    let point2 = Point { x : 3.5 , y : 4.7 } ;
    let notes = Notes { s5 : 10 , s6 : 17.15} ;  

    println!("value of x is {}", point1.y);

    let point2 = Point2::mixup(Point2 {x:1,y:3}, Point2 {x:4,y:3} ); 
}

/*fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}*/