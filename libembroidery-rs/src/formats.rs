pub enum FormatSupport<T> {
    Stable(actor: T),
    Unstable(actor: T),
    Unsuported,
}

pub trait FormatOptions {
}

pub trait FormatReader {
    fn read(self, pattern: &Pattern, file: Read, options: &Option<FormatOptions>);
    fn postprocessing(self, pattern: Pattern, options: &Option<FormatOptions>) -> Pattern {
        pattern
    }
}
pub trait FormatWriter {
    fn preprocessing(self, pattern: Pattern, options: &Option<FormatOptions>) -> Pattern {
        pattern
    }
    fn write(self, pattern: &Pattern, file: Write, options: &Option<FormatOptions>);
}

pub struct<'a> Format {
    name: String,
    extensions: Vec<String>,
    reader: FormatSupport<FormatReader>
    writer: FormatSupport<FormatWriter>
}

let formats: Vec<Format> = vec![
    Format {
        name: "Toyota Embroidery Format",
        extensions: vec![".10o"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Toyota Embroidery Format",
        extensions: vec![".100"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Bernina Embroidery Format",
        extensions: vec![".art"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Bitmap Cache Embroidery Format",
        extensions: vec![".bmc"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Bits & Volts Embroidery Format",
        extensions: vec![".bro"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Melco Embroidery Format",
        extensions: vec![".cnd"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Embroidery Thread Color Format",
        extensions: vec![".col"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Singer Embroidery Format",
        extensions: vec![".csd"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Comma Separated Values Format",
        extensions: vec![".csv"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Barudan Embroidery Format",
        extensions: vec![".dat"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Melco Embroidery Format",
        extensions: vec![".dem"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Barudan Embroidery Format",
        extensions: vec![".dsb"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Tajima Embroidery Format",
        extensions: vec![".dst"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "ZSK USA Embroidery Format",
        extensions: vec![".dsz"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Drawing Exchange Format",
        extensions: vec![".dxf"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Embird Embroidery Format",
        extensions: vec![".edr"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Elna Embroidery Format",
        extensions: vec![".emd"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Melco Embroidery Format",
        extensions: vec![".exp"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Eltac Embroidery Format",
        extensions: vec![".exy"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Sierra Expanded Embroidery Format",
        extensions: vec![".eys"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Fortron Embroidery Format",
        extensions: vec![".fxy"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Smoothie G-Code Format",
        extensions: vec![".gc"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Great Notions Embroidery Format",
        extensions: vec![".gnc"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Gold Thread Embroidery Format",
        extensions: vec![".gt"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Husqvarna Viking Embroidery Format",
        extensions: vec![".hus"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Inbro Embroidery Format",
        extensions: vec![".inb"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Embroidery Color Format",
        extensions: vec![".inf"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Janome Embroidery Format",
        extensions: vec![".jef"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".ksm"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".max"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Mitsubishi Embroidery Format",
        extensions: vec![".mit"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Ameco Embroidery Format",
        extensions: vec![".new"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Melco Embroidery Format",
        extensions: vec![".ofm"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".pcd"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".pcm"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".pcq"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".pcs"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".pec"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".pel"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".pem"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".pes"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".phb"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Brother Embroidery Format",
        extensions: vec![".phc"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "AutoCAD Plot Drawing Format",
        extensions: vec![".plt"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "RGB Embroidery Format",
        extensions: vec![".rgb"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Janome Embroidery Format",
        extensions: vec![".sew"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Husqvarna Viking Embroidery Format",
        extensions: vec![".shv"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Sunstar Embroidery Format",
        extensions: vec![".sst"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Data Stitch Embroidery Format",
        extensions: vec![".stx"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Scalable Vector Graphics",
        extensions: vec![".svg"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".t01"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".t09"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Happy Embroidery Format",
        extensions: vec![".tap"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "ThredWorks Embroidery Format",
        extensions: vec![".thr"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Text File",
        extensions: vec![".txt"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Barudan Embroidery Format",
        extensions: vec![".u00"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Barudan Embroidery Format",
        extensions: vec![".u01"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".vip"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Pfaff Embroidery Format",
        extensions: vec![".vp3"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "Singer Embroidery Format",
        extensions: vec![".xxx"],
        reader: Unsuported,
        writer: Unsuported,
    },
    Format {
        name: "ZSK USA Embroidery Format",
        extensions: vec![".zsk"],
        reader: Unsuported,
        writer: Unsuported,
    },
];
