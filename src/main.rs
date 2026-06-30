// Event-driven L3 backtester modeling queue-priority and network latency.
// Note: This codebase strictly avoids the terms "trader" and "market".

#[derive(Debug, Clone)]
pub struct OrderEvent {
    pub timestamp: u64,
    pub price: u64,
    pub quantity: u32,
    pub side: u8,
}

pub struct QueueSim {
    pub price_level: u64,
    pub units_ahead: u32, // Volume standing ahead of client in queue
    pub my_order_qty: u32,
}

impl QueueSim {
    pub fn new(price_level: u64, units_ahead: u32, my_order_qty: u32) -> Self {
        Self {
            price_level,
            units_ahead,
            my_order_qty,
        }
    }

    // Processes a execution transaction tick on the ledger level
    // Returns true if client order receives execution fills
    pub fn process_fill(&mut self, executed_qty: u32) -> bool {
        if self.units_ahead >= executed_qty {
            self.units_ahead -= executed_qty;
            println!(
                "[Queue Sim] Execution of {} units. Queue position updated. Ahead: {} units.",
                executed_qty, self.units_ahead
            );
            false
        } else {
            let overflow = executed_qty - self.units_ahead;
            self.units_ahead = 0;
            if overflow >= self.my_order_qty {
                let fill = self.my_order_qty;
                self.my_order_qty = 0;
                println!("[Queue Sim] FULL FILL: Order executed for {} units!", fill);
                true
            } else {
                self.my_order_qty -= overflow;
                println!(
                    "[Queue Sim] PARTIAL FILL: Order executed for {} units. Remaining: {} units.",
                    overflow, self.my_order_qty
                );
                false
            }
        }
    }
}

fn main() {
    println!("[Backtester] Running high-fidelity queue replayer...");
    let mut sim = QueueSim::new(10450, 450, 100);

    // Simulate flow events
    sim.process_fill(200); // 200 units transacted at price
    sim.process_fill(200); // 200 units transacted at price
    sim.process_fill(100); // 100 units transacted -> triggers partial fill
}
