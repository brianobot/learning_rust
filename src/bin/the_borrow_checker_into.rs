fn main() {
    // the borrow checker checks that all access to a data is legal
    // thereby ensuring safety in rust programs

    // learning how this works would at least speed up your development time
    // as you will have to face/deal with less complains from the compiler
    // it will also build up your confidence in handling larger projects
    // and higher concepts like concurrency etc

    // borrow checking relies on 3 inter-related concepts
    // - lifetimes
    // - ownership
    // - borrowing

    // ownership refers to the property of variables to clean up their data
    // when they are no longer needed, it does not mean that the owners do not
    // allow access to their data, but that they are in charge of the clean process
    // when it is needed, an owner cleans up it value when it lifetime ends,
    // the compiler actually injects destructors calls to free memory when the owner
    // needs to clean up it value memory trace

    // lifetime is the period of time when accessing a value is valid behaviour

    // borrowing means to access a value, but not take charge of it clean up process
    // in these concepts the borrower is not obligated to directly return the value as 
    // the term suggests, you can think of it as borrowring access to a value

    // Example to demonstrate the concept of the borrow checker
    #![allow(unused_variables, dead_code)]
    
    #[derive(Debug, Clone)]
    struct Message {
        to: u64,
        content: String,
    }

    #[derive(Debug)]
    struct MailBox {
        messages: Vec<Message>,
    }

    #[derive(Debug)]
    struct CubeSat {
        id: u64,
        mailbox: MailBox,
    }

    #[derive(Debug)]
    struct GroundStation {
        mailbox: MailBox,
        radio_freq: f32,
    }

    impl MailBox {
        fn post(&mut self, msg: Message) {
            self.messages.push(msg);
        }
    }

    impl CubeSat {
        fn new(id: u64) -> Self {
            CubeSat {
                id,
                mailbox: MailBox { messages: Vec::new() },
            }
        }

        fn recv(&mut self) -> Option<Message> {
            self.mailbox.messages.pop()
        }
    }

    impl GroundStation {
        fn new() -> Self {
            GroundStation {
                mailbox: MailBox { messages: Vec::new() },
                radio_freq: 99.23,
            }
        }

        fn send(&self, to: &mut CubeSat, msg: Message) -> usize {
            to.mailbox.post(msg.clone());
            msg.content.len()
        }

        fn connect(&self, sat_id: u64) -> CubeSat {
            CubeSat::new(sat_id)
        }
    }

    let sat_a: CubeSat = CubeSat::new(1);
    let sat_b: CubeSat = CubeSat::new(2);
    let sat_c: CubeSat = CubeSat::new(3);

    #[derive(Debug)]
    enum StatusMessage {
        Ok,
    }

    fn check_status(sat_id: CubeSat) -> CubeSat {
        // here achieve the process through a side effect and return 
        // the move value to the caller of the function
        println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
        sat_id
    }

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // "waiting" ...
    let mut sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // movement in rust, refer to the moment of ownership and not physical movement of data
    // primitive types have the copy semantics while other types have the move semantics
    // this allows primitives types like integers to be copied incases which might look like a move

    // to implement a custom destructor, implement the Drop trait on your type/
    // the Drop trait has one method drop, which can contain logic to clean up the memory for the value being dropped
    // it is usually used when an unsafe code is used to allocate memory

    // there are 2 ways to move ownership of a value in rust
    // 1- Through assignment
    // 2- Through the function call portal

    // use these 4 points to better work with ownership
    // - use references where full ownership is not required
    // - duplicate values
    // - refactor code to reduce number of long-lived objects
    // - wrap your objects in a type designed to aid movement

    let base = GroundStation::new();

    let msg_1 = Message{ to: 1, content: String::from("May Day!") };
    let msg_1_sent = base.send(&mut sat_a, msg_1);

    println!("{:?}", sat_a);

    let msg_recv_1 = sat_a.recv();
    let msg_recv_2 = sat_a.recv();

    println!("Msg recv = {:?}", msg_recv_1);
    println!("Msg recv = {:?}", msg_recv_2);

    fn fetch_sat_ids() -> Vec<u64> {
        vec![1, 2, 3]
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = Message{ to: 1, content: String::from("May Day!") };

        let msg_sent = base.send(&mut sat, msg);
        let msg_recv = sat.recv();

        println!("Message Sent = {}", msg_sent);
    }

    // to implement Copy on a type, the fields of the type must also implement copy
    // like mentioned above, the primitve types like integer implement cop by default

    // Copy is implicit 
    // Clone is explicit

    // Rc - Reference Counted type
    use std::rc::Rc;
    use std::cell::RefCell;

    let base_new = Rc::new(GroundStation::new());

    println!("Base New = {:?}", base_new);

    // each time .clone is called on the rc type a count is incremented
    // and each time Drop is called on the rc type, the count is decremented
    // when the count drops to 0, the original type is freed from memory

    // rc does not permit mutation, to allow for mutation
    // it must be wrapped in the RefCell type 

    let base = Rc::new(RefCell::new(GroundStation::new()));

    println!("Base = {:?}", base);
 
    // Rc is not threadsafe, replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>> 


}