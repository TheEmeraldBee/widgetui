#[macro_export]
macro_rules! layout {
    ($size:expr, $(($($line:tt)*) $(=> { $($row:tt)* })? ),+) => {{
        let chunks = Layout::new().direction(Direction::Vertical).constraints([
            $(constraint!($($line)*)),+
        ]).split($size);

        let mut results = vec![vec![]; chunks.len()];

        let mut idx = -1;

        $(
            idx += 1;
            let i = idx as usize;
            results[i] = vec![chunks[i]];
            $(
                let row_constraints = layout!(@row $($row)*);
                let row = Layout::new().direction(Direction::Horizontal).constraints(
                    row_constraints
                ).split(chunks[i]);

                results[i].clear();

                results[i].append(&mut Vec::from(row.as_ref().clone()));
            )?
        )+

        results
    }};

    (@row %$val:expr $(, $($row:tt)*)?) => {{
        let mut constraint = vec![constraint!(%$val)];
        $(constraint.append(&mut layout!(@row $($row)*));)?
        constraint
    }};

    (@row #$val:expr $(, $($row:tt)*)?) => {{
        let mut constraint = vec![constraint!(#$val)];
        $(constraint.append(&mut layout!(@row $($row)*));)?
        constraint
    }};

    (@row >$val:expr $(, $($row:tt)*)?) => {{
        let mut constraint = vec![constraint!(>$val)];
        $(constraint.append(&mut layout!(@row $($row)*));)?
        constraint
    }};

    (@row <$val:expr $(, $($row:tt)*)?) => {{
        let mut constraint = vec![constraint!(<$val)];
        $(constraint.append(&mut layout!(@row $($row)*));)?
        constraint
    }};

    (@row $val:expr ; $val2:expr $(, $($row:tt)*)?) => {{
        let mut constraint = vec![constraint!($val ; $val2)];
        $(constraint.append(&mut layout!(@row $($row)*));)?
        constraint
    }};


}

#[macro_export]
macro_rules! constraint {
    (%$val:expr) => {
        Constraint::Percentage($val)
    };
    (#$val:expr) => {
        Constraint::Length($val)
    };
    (>$val:expr) => {
        Constraint::Min($val)
    };
    (<$val:expr) => {
        Constraint::Max($val)
    };
    ($val:expr ; $val2:expr) => {
        Constraint::Ratio($val, $val2)
    };
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use ratatui::prelude::*;
    #[test]
    fn constraints() {
        let constraint = constraint!(%50);
        assert_eq!(Constraint::Percentage(50), constraint);

        let constraint = constraint!(#50);
        assert_eq!(Constraint::Length(50), constraint);

        let constraint = constraint!(>50);
        assert_eq!(Constraint::Min(50), constraint);

        let constraint = constraint!(<50);
        assert_eq!(Constraint::Max(50), constraint);

        let constraint = constraint!(50;10);
        assert_eq!(Constraint::Ratio(50, 10), constraint);
    }

    #[test]
    fn layout() {
        let popup = layout![
            Rect::new(0, 0, 1000, 1000),
            (%50),
            (>3) => {
                %10,
                %80,
                %10
            },
            (%50)
        ][1][1];

        let popup_test = Rect::new(26, 128, 204, 3);

        assert_eq!(popup, popup_test);
    }
}
