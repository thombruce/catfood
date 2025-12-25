#[derive(Debug)]
pub struct Sparkline {
    pub enabled: bool,
    pub length: usize,
    pub data: Vec<u64>,
    pub logarithmic: bool,
}

impl Sparkline {
    pub fn new(enabled: bool, length: usize, logarithmic: bool) -> Self {
        Self {
            enabled,
            length,
            data: vec![0; length],
            logarithmic,
        }
    }

    pub fn update(&mut self, value: u64) {
        if self.enabled {
            self.data.remove(0);
            self.data.push(value);
        }
    }

    pub fn render(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let max_value = self.data.iter().max().unwrap_or(&1);
        if *max_value == 0 {
            return " ".repeat(self.length);
        }

        let bars = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        let mut result = String::new();

        for &value in &self.data {
            if value == 0 {
                result.push(' ');
            } else {
                let normalized = if self.logarithmic {
                    // Log scale: log10(value) / log10(max_value)
                    (value as f64).log10() / (*max_value as f64).log10()
                } else {
                    // Linear scale (current behavior)
                    value as f64 / *max_value as f64
                };
                let index = (normalized * (bars.len() - 1) as f64) as usize;
                result.push(bars[index.min(bars.len() - 1)].chars().next().unwrap());
            }
        }

        result
    }

    pub fn render_with_spaces(&self) -> String {
        if self.enabled {
            self.render()
        } else {
            " ".repeat(self.length)
        }
    }
}
