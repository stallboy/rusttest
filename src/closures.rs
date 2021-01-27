struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn generate_workout() {
    let mut _expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });
}

#[test]
fn test_call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}

#[test]
fn test_capture_env() {
    let x = 4;
    let equal_to_x = |z: i32| -> bool { z == x };
    let y = 4;
    assert!(equal_to_x(y));

    // let mut zz = 55;
    // let set_zz  =  |z| { zz = z; };
    // set_zz(5);
    // println!("{}", zz);
}
