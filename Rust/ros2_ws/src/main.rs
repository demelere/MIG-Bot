use r2r::{Context, Node, Publisher, std_msgs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::create()?;
    let mut node = Node::create(context, "my_robot_node")?;

    let publisher = node.create_publisher::<std_msgs::msg::String>("topic_name")?;

    let mut message = std_msgs::msg::String {
        data: "Hello, MIG Bot!".to_string(),
    };

    loop {
        publisher.publish(&message)?;
        println!("Published: {}", message.data);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
