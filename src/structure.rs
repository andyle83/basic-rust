#[cfg(test)]
mod structure {
    /// A rectangle of eight-bit grayscale pixel
    struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }

    impl GrayscaleMap {
        pub fn friendly_print(&self, label: &str) {
            println!("*** image {:} ***", label);
            println!("pixels size {}", self.pixels.len());
            println!("size {:?}", self.size);
            println!("*** end ***");
        }
    }

    // shorthand syntax for structure
    fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { pixels, size }
    }

    #[test]
    fn test_structure() {
        let width = 1024;
        let height = 576;
        let image1 = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height)
        };
        assert_eq!(image1.pixels.len(), width * height);

        let image2 = new_map((width, height), vec![1; width * height]);
        assert_eq!(image2.pixels.len(), width * height);

        // calling struct method
        image1.friendly_print("image1");
        image2.friendly_print("image2");
    }

}