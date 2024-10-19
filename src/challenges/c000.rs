use crate::challenges::c001;
use crate::monster::{ask, redraw, say, warn};

pub fn setup() {
    redraw();

    say(r#"
    Welcome, stranger. My name is Scylla, the NoSQL monster. <nom nom> 
    I'm here to guide you through a series of challenges to make learning ScyllaDB fun!
    As you make progress I will award you some monster coins 🪙🪙🪙

    You can then use those coins to get some cool merchandise on our store.

    "#);

    if ask("Are you ready for the first challenge?") {
        c001::setup();
    } else {
        warn("No worries, you can come back and try another time!");
    }
}
