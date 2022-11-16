fn examples() {
    find_best_move_t_tt_id(
        &current_move,
        Duration::from_millis(1000),
        color,
        &mut table,
        |state, depth, color, table| {
            -nega_with_table(state, depth, color, table, isize::MIN + 1, isize::MAX)
        },
    );

    find_best_move(current_move, set_depth + 1, color, |state, depth, color| {
        -nega(&state, depth, color, isize::MIN + 1, isize::MAX)
    });

    find_best_move(current_move, set_depth + 1, color, |state, depth, color| {
        -nega_scout(&state, depth, color, isize::MIN + 1, isize::MAX)
    });

    find_best_move_t_tt(
        &current_move,
        Duration::from_millis(1000),
        color,
        &mut table,
        |state, color, max_time, table| {
            iterative_deepening_t_tt(
                state,
                color,
                max_time,
                table,
                |state, depth, color, mut table| {
                    alpha_beta_with_table(
                        state,
                        depth,
                        color,
                        &mut table,
                        isize::MIN + 1,
                        isize::MAX,
                    )
                },
            )
        },
    );

    find_best_move_tt(
        &current_move,
        set_depth + 2,
        color,
        &mut table,
        |state, depth, color, table| {
            id_tt(
                state,
                depth,
                color,
                table,
                |state, depth, color, mut table| {
                    alpha_beta_with_table(
                        state,
                        depth,
                        color,
                        &mut table,
                        isize::MIN + 1,
                        isize::MAX,
                    )
                },
            )
        },
    );
    find_best_move_tt(
        &current_move,
        set_depth + 2,
        color,
        &mut table,
        |state, depth, color, mut table| {
            alpha_beta_with_table(&state, depth, color, &mut table, isize::MIN + 1, isize::MAX)
        },
    );
    random_agent(current_move, color, &mut seed);
    current_move.choose_move(color)
}
