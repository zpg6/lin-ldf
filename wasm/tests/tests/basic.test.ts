import { describe, it, expect, beforeAll } from "bun:test";
import init, {
    parse_ldf_file,
    get_ldf_stats,
    validate_ldf,
    parse_ldf_to_json,
    type LinLdf,
    type LdfStats,
} from "lin-ldf";

// LDF example from README
const testLdf = `
LIN_description_file ;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;

/* PARSING IGNORES BLOCK COMMENTS */
// AND LINE COMMENTS

Nodes {
    Master: Master, 5 ms, 0.1 ms ;
    Slaves: Slave1, Slave2, Slave3 ;
}

Signals {
    Signal1: 10, 0, Master, Slave1 , Slave2 ;
    Signal2: 10, 0, Master, Slave1 ;
    Signal3: 10, 0, Slave1, Master ;
    Signal4: 10, 0, Slave1, Master ;
    Signal5: 2, 0, Slave1, Master ;
    Signal6: 1, 0, Slave1, Master ;
}

Frames {
    Frame1: 0, Master, 8 {
        Signal1, 0 ;
        Signal2, 10 ;
    }
    Frame2: 0x16, Slave1, 8 {
        Signal3, 0 ;
        Signal4, 10 ;
    }
}

Node_attributes {
   Slave1 {
       LIN_protocol = "2.1" ;
       configured_NAD = 0xB ;
       initial_NAD = 0xB ;
       product_id = 0x123, 0x4567, 8 ;
       response_error = Signal1 ;
       P2_min = 100 ms ;
       ST_min = 0 ms ;
       N_As_timeout = 1000 ms ;
       N_Cr_timeout = 1000 ms ;
       configurable_frames {
           Frame1 ;
           Frame2 ;
       }
   }
   Slave2 {
       LIN_protocol = "2.1" ;
       configured_NAD = 0xC ;
       initial_NAD = 0xC ;
       product_id = 0x124, 0x4568, 0x66 ;
       response_error = Signal2 ;
       P2_min = 100 ms ;
       ST_min = 0 ms ;
       N_As_timeout = 1000 ms ;
       N_Cr_timeout = 1000 ms ;
       configurable_frames {
           Frame1 ;
           Frame2 ;
       }
   }
}

Schedule_tables {
    AllFrames {
        Frame1 delay 10 ms ;
        Frame2 delay 10 ms ;
    }
}
`;

describe("lin-ldf WASM package", () => {
    beforeAll(async () => {
        await init();
    });

    it("should validate LDF content", () => {
        const isValid = validate_ldf(testLdf);
        expect(isValid).toBe(true);
    });

    it("should parse LDF file with full type safety", () => {
        const ldf: LinLdf = parse_ldf_file(testLdf);

        // Verify header
        expect(ldf.header.lin_protocol_version).toBe("2.1");
        expect(ldf.header.lin_language_version).toBe("2.1");
        expect(ldf.header.lin_speed).toBe(19200);

        // Verify nodes
        expect(ldf.nodes.master.name).toBe("Master");
        expect(ldf.nodes.master.time_base).toBe("5 ms");
        expect(ldf.nodes.master.jitter).toBe("0.1 ms");
        expect(ldf.nodes.slaves).toHaveLength(3);
        expect(ldf.nodes.slaves[0].name).toBe("Slave1");
        expect(ldf.nodes.slaves[1].name).toBe("Slave2");
        expect(ldf.nodes.slaves[2].name).toBe("Slave3");

        // Verify signals
        expect(ldf.signals).toHaveLength(6);
        expect(ldf.signals[0].name).toBe("Signal1");
        expect(ldf.signals[0].signal_size).toBe(10);
        expect(ldf.signals[0].published_by).toBe("Master");
        expect(ldf.signals[0].subscribed_by).toContain("Slave1");
        expect(ldf.signals[0].subscribed_by).toContain("Slave2");

        // Verify frames
        expect(ldf.frames).toHaveLength(2);
        expect(ldf.frames[0].frame_name).toBe("Frame1");
        expect(ldf.frames[0].frame_id).toBe(0);
        expect(ldf.frames[0].published_by).toBe("Master");
        expect(ldf.frames[0].frame_size).toBe(8);
        expect(ldf.frames[0].signals).toHaveLength(2);

        expect(ldf.frames[1].frame_name).toBe("Frame2");
        expect(ldf.frames[1].frame_id).toBe(0x16);
        expect(ldf.frames[1].published_by).toBe("Slave1");
    });

    it("should handle signal init values with proper types", () => {
        const ldf: LinLdf = parse_ldf_file(testLdf);

        for (const signal of ldf.signals) {
            // Type-safe enum handling
            if ("Scalar" in signal.init_value) {
                expect(typeof signal.init_value.Scalar).toBe("number");
                expect(signal.init_value.Scalar).toBe(0);
            } else {
                expect(Array.isArray(signal.init_value.Array)).toBe(true);
            }
        }
    });

    it("should provide detailed statistics", () => {
        const stats: LdfStats = get_ldf_stats(testLdf);

        // The stats object is actually a Map, so we need to use .get() method
        expect(typeof stats).toBe("object");
        expect(stats).toBeDefined();

        // Access Map properties using .get() method
        expect((stats as any).get("signal_count")).toBe(6);
        expect((stats as any).get("frame_count")).toBe(2);
        expect((stats as any).get("node_count")).toBe(4); // Master + 3 slaves
        expect((stats as any).get("schedule_table_count")).toBe(1);
        expect((stats as any).get("node_attributes_count")).toBe(2);
        expect((stats as any).get("lin_protocol_version")).toBe("2.1");
        expect((stats as any).get("lin_language_version")).toBe("2.1");
        expect((stats as any).get("lin_speed")).toBe(19200);
    });

    it("should parse to JSON and back", () => {
        const jsonStr = parse_ldf_to_json(testLdf);
        const parsed = JSON.parse(jsonStr);

        expect(parsed.header.lin_protocol_version).toBe("2.1");
        expect(parsed.signals).toHaveLength(6);
        expect(parsed.frames).toHaveLength(2);
    });

    it("should handle node attributes correctly", () => {
        const ldf: LinLdf = parse_ldf_file(testLdf);

        expect(ldf.node_attributes).toHaveLength(2);

        const slave1 = ldf.node_attributes.find(na => na.node_name === "Slave1");
        expect(slave1).toBeDefined();
        expect(slave1!.lin_protocol).toBe("2.1");
        expect(slave1!.configured_nad).toBe(0xb);
        expect(slave1!.initial_nad).toBe(0xb);
        expect(slave1!.supplier_id).toBe(0x123);
        expect(slave1!.function_id).toBe(0x4567);
        expect(slave1!.variant).toBe(8);
        expect(slave1!.response_error).toBe("Signal1");
        expect(slave1!.configurable_frames).toContain("Frame1");
        expect(slave1!.configurable_frames).toContain("Frame2");

        const slave2 = ldf.node_attributes.find(na => na.node_name === "Slave2");
        expect(slave2).toBeDefined();
        expect(slave2!.configured_nad).toBe(0xc);
        expect(slave2!.supplier_id).toBe(0x124);
        expect(slave2!.function_id).toBe(0x4568);
        expect(slave2!.variant).toBe(0x66);
    });

    it("should handle schedule tables correctly", () => {
        const ldf: LinLdf = parse_ldf_file(testLdf);

        expect(ldf.schedule_tables).toHaveLength(1);
        expect(ldf.schedule_tables[0].schedule_table_name).toBe("AllFrames");
        expect(ldf.schedule_tables[0].frame_delays).toHaveLength(2);

        const frame1Delay = ldf.schedule_tables[0].frame_delays.find(fd => fd.frame_name === "Frame1");
        expect(frame1Delay).toBeDefined();
        expect(frame1Delay!.frame_time).toBe(10.0);

        const frame2Delay = ldf.schedule_tables[0].frame_delays.find(fd => fd.frame_name === "Frame2");
        expect(frame2Delay).toBeDefined();
        expect(frame2Delay!.frame_time).toBe(10.0);
    });

    it("should handle frame signals correctly", () => {
        const ldf: LinLdf = parse_ldf_file(testLdf);

        const frame1 = ldf.frames.find(f => f.frame_name === "Frame1");
        expect(frame1).toBeDefined();
        expect(frame1!.signals).toHaveLength(2);

        const signal1InFrame = frame1!.signals.find(s => s.signal_name === "Signal1");
        expect(signal1InFrame).toBeDefined();
        expect(signal1InFrame!.start_bit).toBe(0);

        const signal2InFrame = frame1!.signals.find(s => s.signal_name === "Signal2");
        expect(signal2InFrame).toBeDefined();
        expect(signal2InFrame!.start_bit).toBe(10);
    });
});
