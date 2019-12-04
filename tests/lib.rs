#[cfg(test)]
mod tests {
    use app::models::Human;
    use app::models::Mammals;

    #[test]
    fn point() {
        let p = Human {
            age: 1
        };
        p.do_mammals_work();
    }

}
