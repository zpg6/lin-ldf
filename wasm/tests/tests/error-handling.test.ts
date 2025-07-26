import { describe, it, expect, beforeAll } from "bun:test";
import init, { parse_ldf_file, validate_ldf, get_ldf_stats } from "lin-ldf";

describe("lin-ldf error handling", () => {
    beforeAll(async () => {
        await init();
    });

    it("should reject invalid LDF content", () => {
        const invalidLdf = "This is not a valid LDF file";

        expect(validate_ldf(invalidLdf)).toBe(false);

        expect(() => {
            parse_ldf_file(invalidLdf);
        }).toThrow();
    });

    it("should reject malformed LDF header", () => {
        const malformedLdf = `
      NOT_LIN_description_file ;
      LIN_protocol_version = "2.1" ;
    `;

        expect(validate_ldf(malformedLdf)).toBe(false);

        expect(() => {
            parse_ldf_file(malformedLdf);
        }).toThrow();
    });

    it("should reject incomplete LDF file", () => {
        const incompleteLdf = `
      LIN_description_file ;
      LIN_protocol_version = "2.1" ;
      LIN_language_version = "2.1" ;
      LIN_speed = 19.2 kbps ;
      
      Nodes {
        Master: Master, 5 ms, 0.1 ms ;
        Slaves: Slave1 ;
      }
      // Missing required sections
    `;

        expect(validate_ldf(incompleteLdf)).toBe(false);

        expect(() => {
            parse_ldf_file(incompleteLdf);
        }).toThrow();
    });

    it("should handle empty content", () => {
        const emptyContent = "";

        expect(validate_ldf(emptyContent)).toBe(false);

        expect(() => {
            parse_ldf_file(emptyContent);
        }).toThrow();
    });

    it("should handle whitespace-only content", () => {
        const whitespaceOnly = "   \n\t   \n  ";

        expect(validate_ldf(whitespaceOnly)).toBe(false);

        expect(() => {
            parse_ldf_file(whitespaceOnly);
        }).toThrow();
    });

    it("should handle stats request on invalid content", () => {
        const invalidLdf = "Invalid content";

        expect(() => {
            get_ldf_stats(invalidLdf);
        }).toThrow();
    });
});
