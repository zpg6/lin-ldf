import { describe, it, expect, beforeAll } from "bun:test";
import init, {
    parse_ldf_file,
    type LinLdf,
    type LdfSignal,
    type LdfFrame,
    type LdfHeader,
    type LdfNodes,
} from "lin-ldf";

// Minimal LDF for type testing
const minimalLdf = `
LIN_description_file ;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;

Nodes {
    Master: Master, 5 ms, 0.1 ms ;
    Slaves: Slave1 ;
}

Signals {
    TestSignal: 8, 42, Master, Slave1 ;
}

Frames {
    TestFrame: 0x10, Master, 8 {
        TestSignal, 0 ;
    }
}

Node_attributes {}
Schedule_tables {}
`;

describe("TypeScript type safety", () => {
    beforeAll(async () => {
        await init();
    });

    it("should provide proper LinLdf type structure", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);

        // Type assertions to verify structure
        expect(typeof ldf).toBe("object");
        expect(ldf).toHaveProperty("header");
        expect(ldf).toHaveProperty("nodes");
        expect(ldf).toHaveProperty("signals");
        expect(ldf).toHaveProperty("frames");
        expect(ldf).toHaveProperty("diagnostic_signals");
        expect(ldf).toHaveProperty("diagnostic_frames");
        expect(ldf).toHaveProperty("node_attributes");
        expect(ldf).toHaveProperty("schedule_tables");
        expect(ldf).toHaveProperty("signal_encoding_types");
        expect(ldf).toHaveProperty("signal_representations");
    });

    it("should provide proper LdfHeader type", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);
        const header: LdfHeader = ldf.header;

        expect(typeof header.lin_protocol_version).toBe("string");
        expect(typeof header.lin_language_version).toBe("string");
        expect(typeof header.lin_speed).toBe("number");
        expect(
            header.channel_name === null || header.channel_name === undefined || typeof header.channel_name === "string"
        ).toBe(true);
    });

    it("should provide proper LdfNodes type", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);
        const nodes: LdfNodes = ldf.nodes;

        expect(typeof nodes.master.name).toBe("string");
        expect(typeof nodes.master.time_base).toBe("string");
        expect(typeof nodes.master.jitter).toBe("string");

        expect(Array.isArray(nodes.slaves)).toBe(true);
        nodes.slaves.forEach(slave => {
            expect(typeof slave.name).toBe("string");
        });
    });

    it("should provide proper LdfSignal type", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);
        const signal: LdfSignal = ldf.signals[0];

        expect(typeof signal.name).toBe("string");
        expect(typeof signal.signal_size).toBe("number");
        expect(typeof signal.published_by).toBe("string");
        expect(Array.isArray(signal.subscribed_by)).toBe(true);

        // Type-safe enum access
        if ("Scalar" in signal.init_value) {
            expect(typeof signal.init_value.Scalar).toBe("number");
            expect(signal.init_value.Scalar).toBe(42);
        } else {
            expect(Array.isArray(signal.init_value.Array)).toBe(true);
        }
    });

    it("should provide proper LdfFrame type", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);
        const frame: LdfFrame = ldf.frames[0];

        expect(typeof frame.frame_name).toBe("string");
        expect(typeof frame.frame_id).toBe("number");
        expect(typeof frame.published_by).toBe("string");
        expect(typeof frame.frame_size).toBe("number");
        expect(Array.isArray(frame.signals)).toBe(true);

        frame.signals.forEach(frameSignal => {
            expect(typeof frameSignal.signal_name).toBe("string");
            expect(typeof frameSignal.start_bit).toBe("number");
        });
    });

    it("should handle arrays with proper typing", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);

        // All arrays should be properly typed
        expect(Array.isArray(ldf.signals)).toBe(true);
        expect(Array.isArray(ldf.frames)).toBe(true);
        expect(Array.isArray(ldf.diagnostic_signals)).toBe(true);
        expect(Array.isArray(ldf.diagnostic_frames)).toBe(true);
        expect(Array.isArray(ldf.node_attributes)).toBe(true);
        expect(Array.isArray(ldf.schedule_tables)).toBe(true);
        expect(Array.isArray(ldf.signal_encoding_types)).toBe(true);
        expect(Array.isArray(ldf.signal_representations)).toBe(true);

        // Each array element should have proper structure
        ldf.signals.forEach((signal: LdfSignal) => {
            expect(typeof signal.name).toBe("string");
        });

        ldf.frames.forEach((frame: LdfFrame) => {
            expect(typeof frame.frame_name).toBe("string");
        });
    });

    it("should maintain type safety through transformations", () => {
        const ldf: LinLdf = parse_ldf_file(minimalLdf);

        // Type-safe filtering
        const masterFrames = ldf.frames.filter(frame => frame.published_by === "Master");
        masterFrames.forEach(frame => {
            expect(typeof frame.frame_name).toBe("string");
            expect(frame.published_by).toBe("Master");
        });

        // Type-safe mapping
        const signalNames = ldf.signals.map(signal => signal.name);
        signalNames.forEach(name => {
            expect(typeof name).toBe("string");
        });

        // Type-safe reduction
        const totalSignalBits = ldf.signals.reduce((total, signal) => total + signal.signal_size, 0);
        expect(typeof totalSignalBits).toBe("number");
        expect(totalSignalBits > 0).toBe(true);
    });
});
