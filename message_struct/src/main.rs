#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, tid)) => println!("User {:?} joins topic {:?}", uid, tid),
        Event::Leave((uid, tid)) => println!("User {:?} leaves topic {:?}", uid, tid),
        Event::Message((uid, tid, msg)) => println!("User {:?} sends message in topic {:?}, message body: {:?}", uid, tid, msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((uid, tid, msg)) = event {
        println!("User {:?} sends message in topic {:?}, message body: {:?}", uid, tid, msg);
    }
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

    let topic = Topic { id: TopicId(1), name: "Rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello, world!".into()));
    let event4 = Event::Leave((alice.id, topic.id));

    println!("Evt1: {:?}, Evt2: {:?}, Evt3: {:?}, Evt4: {:?}", event1, event2, event3, event4);
    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
    process_event(&event4);
    process_message(&event3);
}
