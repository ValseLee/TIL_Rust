pub mod rust_programming_concepts {
    /// let은 '변수' 지만 mut 키워드 없이는 불변한다.
    /// 그러나 variable Shadowing이 있다면 같은 이름의 변수를 재활용할 수 있다.
    /// 이 경우 타입의 변환이 가능하다.
    fn _variable_control_shadowing() {
        let my_mutable_variable = "";
        println!("My First Shadowing Variable: {}", my_mutable_variable);

        let my_mutable_variable = "Hi";
        println!("My Second Shadowing Variable: {}", my_mutable_variable);

        let my_mutable_variable = 152;
        println!("My Third Shadowing Variable: {}", my_mutable_variable);
    }

    /// mut 키워드와 함께 쓴다면 여타 프로그래밍 언어의 변수처럼 사용할 수 있다.
    /// 그러나 이 경우, 이름을 대체할 수는 없기 때문에 타입 변경 또한 불가능하다.
    fn _variable_control_mut() {
        let mut my_mutable_variable = "";
        my_mutable_variable = "Hi";
        println!("My Mutable Variable: {}", my_mutable_variable);

        // my_mutable_variable = my_mutable_variable.len();
    }

    /// 상수는 당연히 불가변하며, 대문자 snake 표기한다.
    /// const 키워드로 선언한다.
    /// 모든 상수는 타입을 명시해야 한다.
    fn _const_control() {
        const MY_CONSTANT: &str = "";
    }
}

/// Rust는 scalar 타입과 compound 타입을 갖는다.
pub mod rust_programming_types {
    /// Scalar 타입은 하나의 값으로 표현되며, 정수-실수형, 문자형, 불대수, 문자열 타입을 포괄한다.
    /// 정수형, 실수형 타입 간 사칙 연산이 가능하다.
    fn _types_in_rust_scalar() {
        // addition
        const SUM: i32 = 5 + 10;

        // subtraction
        const DIFFERENCE: f64 = 95.5 - 4.3;

        // multiplication
        const PRODUCT: i32 = 4 * 30;

        // division
        const QUOTIENT: f64 = 56.7 / 32.2;

        // remainder
        const REMAINDER: i32 = 43 % 5;

        // 문자 타입은 작은따옴표
        // 유니코드 스칼라와 문자열은 String에서 더 다루게 될 것
        const CHAR_TYPE: char = 'h';
    }

    /// compound 타입은 서로 다른 타입들을 하나의 타입으로 묶어줄 수 있다.
    /// 대표적으로 튜플과 배열이 있다.
    fn _types_in_rust_compound() {
        const MY_TUPLE: (i32, i32, f64) = (3, 6, 9.0);
        // 튜플 타입은 패턴 매칭으로 꺼내올 수 있다.
        // 이를 튜플의 구조 해체(deconstruction) 라고 한다.
        let (x, y, z) = MY_TUPLE;
        println!("My x is: {x}, My y is: {y}, My z is: {z}"); // 3, 6, 9.0

        // 튜플의 색인도 가능하다.
        let first_value = MY_TUPLE.0;
        println!("First Value is: {first_value}"); // 3

        // 배열은 같은 타입만을 가질 수 있으며, 길이가 '고정'되어 있다.
        // 배열은 '고정적' 이며 stack에 저장되지만, 벡터는 '유동적'이고 heap에 저장된다.
        // 대부분의 경우 벡터 타입을 쓰게 된다.
        // 타입 명시할 때는 [type; count] 형태로 명시한다.
        const MY_ARRAY: [i32; 5] = [0,1,2,3,4];
        
        // 배열의 out of range 가 발생하면 프로그램이 정지한다.
        // rust의 에러 발생은 'panic' 이라고 한다!
    }
}