use crate::block::CBCABlock;

pub fn _dbg_generate_masse_message(
    n: usize, 
    instance_id: &String
) -> Vec<CBCABlock> {
    let mut stock: Vec<CBCABlock> = Vec::new();

    for i in 0..n {
        stock.push(
            CBCABlock::block_creator_message(
                format!("Message {}", i), 
                format!("Me"),
                instance_id.to_string()
            ),
        );
    }

    stock
}

pub fn _dbg_generate_masse_massage_data(
    n: usize
) -> Vec<(String, String)> {
    let mut stock: Vec<(String, String)> = Vec::new();

    for i in 0..n {
        stock.push(
            (
                format!("Message {}", i),
                format!("Me")
            )
        );
    }

    stock
}