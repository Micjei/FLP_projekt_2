// =============================================================================
// Custom Integration Tests — Dungeon Map Scribe
// =============================================================================

use dungeon_scribe::{generate_report, DungeonMap};

/// TEST: Komplexní scénář s více chybami a jejich promítnutím do reportu.
/// Ověřuje, že report správně vypíše "FAILED" a seznam všech chyb s odrážkami,
/// pokud mapa postrádá startovní i cílový bod.
#[test]
fn test_integration_multiple_validation_errors_in_report() {
    // Mapa má podlahu, ale nemá @ (start) ani X (exit)
    let input = "####\n#..#\n####";
    let map = DungeonMap::parse(input).unwrap();

    let report = generate_report(&map);

    // Ověření specifického formátu sekce Validation
    assert!(report.contains("Validation: FAILED\n"));
    assert!(report.contains("  - Missing PlayerStart\n"));
    assert!(report.contains("  - No Exit found\n"));

    // Ověření, že i při selhání validace report pokračuje na Reachable floor
    assert!(report.contains("Reachable floor from player: 0"));
}

/// TEST: Hraniční vstup - mapa o rozměrech 1x1.
/// Ověřuje interakci parsování rozměrů a následného reportu pro minimální možnou mapu.
#[test]
fn test_integration_minimal_map_report() {
    let input = "@";
    let map = DungeonMap::parse(input).unwrap();
    let report = generate_report(&map);

    // Přesná shoda rozměrů a výpisu dlaždic
    assert!(report.contains("Dimensions: 1 x 1\n"));
    assert!(report.contains("  PlayerStart:     1\n"));
    // Jelikož chybí Exit a Floor, validace musí selhat
    assert!(report.contains("Validation: FAILED\n"));
}

/// TEST: Ověření "Kanonického pořadí" a filtrace nulových počtů.
/// Ověřuje, že Tiles sekce neobsahuje typy s nulovým počtem a řadí je správně
/// (např. Trap musí být v seznamu níže než Wall).
#[test]
fn test_integration_report_tile_ordering_and_filtering() {
    // Mapa obsahuje jen Wall (#) a Trap (^)
    let input = "#^";
    let map = DungeonMap::parse(input).unwrap();
    let report = generate_report(&map);

    // Nesmí obsahovat Floor ani jiné, které v mapě nejsou
    assert!(!report.contains("Floor:"));
    assert!(!report.contains("Enemy:"));

    // Musí obsahovat Wall a Trap v tomto pořadí (Wall dříve než Trap)
    let wall_pos = report.find("Wall:").unwrap();
    let trap_pos = report.find("Trap:").unwrap();
    assert!(
        wall_pos < trap_pos,
        "Wall must be listed before Trap per canonical order"
    );
}

/// TEST: Chování na hraničním vstupu - mapa s Windows konci řádků (\r\n).
/// Ověřuje, že parsování korektně spolupracuje s výpočtem rozměrů i v případě
/// netriviálních konců řádků, které by mohly ovlivnit width/height.
#[test]
fn test_integration_windows_newlines_handling() {
    // Přidali jsme podlahu '.' místo jedné zdi, aby byla mapa validní
    let input = "###\r\n#@X\r\n#.#";
    let map = DungeonMap::parse(input).expect("Failed to parse map with CRLF");

    assert_eq!(map.width(), 3);
    assert_eq!(map.height(), 3);

    let report = generate_report(&map);
    assert!(report.contains("Validation: OK\n"));
}

/// TEST: Přesné zarovnání (alignment) v reportu.
/// Ověřuje, že čísla jsou zarovnána doprava na 4 pozice a popisky na 14 pozic.
#[test]
fn test_integration_report_exact_visual_alignment() {
    let input = "##########\n#@.......X\n##########";
    let map = DungeonMap::parse(input).unwrap();
    let report = generate_report(&map);

    // "  Wall:           21" -> 2 mezery, "Wall:" (5) + 9 mezer = 14, pak "  21" (celkem 4 místa)
    // Hledáme přesný řetězec včetně mezer pro ověření FR-7
    assert!(report.contains("  Wall:           21\n"));
    assert!(report.contains("  PlayerStart:     1\n"));
}
