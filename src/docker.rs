pub struct ResourceSummary {
    // Struct representing one Docker resource category, such as images or containers
    name: String,           // resource name, such as "Images"
    count: u32,             // number of resources in that category, such as 12 images
    reclaimable_bytes: u64, // estimated disk space that could be recovered, like 4200 MB
}

impl ResourceSummary {
    pub fn display_text(&self) -> String {
        format!(
            "{}: {} resources, {} MB reclaimable",
            self.name,
            self.count,
            format_bytes(self.reclaimable_bytes),
        )
    }

    pub fn new(name: String, count: u32, reclaimable_bytes: u64) -> Self {
        Self {
            name,
            count,
            reclaimable_bytes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn count(&self) -> &u32 {
        &self.count
    }

    pub fn reclaimable_bytes(&self) -> &u64 {
        &self.reclaimable_bytes
    }
}

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)] // this includes this module only when running tests
mod tests {
    use super::*;

    #[test]
    fn formats_bytes() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn formats_kibibites() {
        assert_eq!(format_bytes(1024), "1.0 KiB");
    }

    #[test]
    fn formats_mebibites() {
        assert_eq!(format_bytes(1024 * 1024), "1.0 MiB");
    }

    #[test]
    fn formats_gibibites() {
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GiB");
    }
}
