// Put the package name at top.
package main

// List the libs to import into the program.
import (
    "fmt"
    "strconv"
    "encoding/json"
)

// The main function.
func main () {
    test_error_equal()
}

// Hello world function.
func test_hello () {
    fmt.Println("hello world")
}

// The entire test for adding and showing two numbers.
func test_add_num () {
    // Go promotes type correctness more than script type languages.
    // Be disciplined first, check with everyone else.
    var result int = add_num(3, 5)
    fmt.Println(result)
}

// Add two numbers and return that number.
func add_num (val_1 int, val_2 int) int {
    return val_1 + val_2
}

// Show a boolean conversion.
func test_boolean () {
    var result bool = true || false
    fmt.Println(result)
}

// Test a string interaction.
func test_string () {
    var str_1 string = "part_one"
    var str_2 string = "part_two"
    var comb string = str_1 + " | " + str_2
    fmt.Println(comb)
}

// Test a for loop.
func test_for_loop () {
    var sum int = 0
    for i := 0; i < 10; i++ {
        sum += i
    } 

    // fmt.Println does the conversion within the function.
    fmt.Println("sum:", sum)
}

// Test out if/else statements.
func test_if_else () {
    var num int = 7
    if num < 4 {
        fmt.Println(num, "is less than 4")
    } else if num < 8 {
        fmt.Println(num, "is less than 8")
    } else {
        fmt.Println(num, "is greater than or equal to 8")
    }
}

// Test out pointers.
func test_pointers () {
    var num int = 12
    var ptr *int = &num
    fmt.Println("num value:", num)
    fmt.Println("ptr dereference value:", *ptr)
    fmt.Println("ptr address value:", ptr)

    num = num + 10
    fmt.Println("new num value:", num)
    fmt.Println("new ptr dereference value:", *ptr)
    fmt.Println("new ptr address value:", ptr)
}

// Test out structs.
type Kuz_struct struct {
    X int
    Y int
    Z *int
}

func test_structs () {
    var num int = 1
    var ptr *int = &num
    var struc Kuz_struct = Kuz_struct{X: 1, Z: ptr}
    fmt.Println("num value:", num)
    fmt.Println("ptr dereference value:", *ptr)
    fmt.Println("ptr address value:", ptr)
    fmt.Println("struc X value:", struc.X)
    fmt.Println("struc Y value:", struc.Y)
    fmt.Println("struc Z dereference value:", *struc.Z)
    fmt.Println("struc Z address value:", struc.Z)

    *ptr = *ptr + 10
    fmt.Println()
    fmt.Println("new num value:", num)
    fmt.Println("new ptr dereference value:", *ptr)
    fmt.Println("new ptr address value:", ptr)
    fmt.Println("new struc X value:", struc.X)
    fmt.Println("new struc Y value:", struc.Y)
    fmt.Println("new struc Z dereference value:", *struc.Z)
    fmt.Println("new struc Z address value:", struc.Z)

    var struc_ptr *Kuz_struct = &struc
    fmt.Println("struc_ptr X value:", struc_ptr.X)
}

// Test out arrays.
func test_arrays () {
    const arr_size int = 10
    var arr [arr_size]int
    fmt.Println("The contents of the array:")
    for i := 0; i < arr_size; i++ {
        fmt.Println("arr index:", i, " | arr[index]:", arr[i])
    }

    fmt.Println()
    fmt.Println("Modified contents of the array:")
    for i := 0; i < arr_size; i++ {
        arr[i] = i
        fmt.Println("arr index:", i, " | arr[index]:", arr[i])
    }
}

// Test out slices.
func test_slices () {
    arr := [6]int {6, 4, 3, 2, 6, 7}
    var slic []int = arr[1 : 3]
    fmt.Println("arr:", arr)
    fmt.Println("slic:", slic)
}

// Test out maps.
func test_maps () {
    // Make the map object.
    var m_dict map[string]int = make(map[string]int)

    // Add a few things into the map.
    m_dict["1"] = 10
    m_dict["2"] = 3

    // Try to query for things in and not in map.
    var sum int = m_dict["1"] + 5
    m_dict["1"] = 13
    var d_sum int = m_dict["1"] + m_dict["2"]
    fmt.Println("sum:", sum)
    fmt.Println("d_sum:", d_sum)
    fmt.Println("m_dict[\"1\"]:", m_dict["1"])

    // If you try to get ["3"], it just assigns the variable 0.
    // It does not assign ["3"] anything.
    var invalid_query int = m_dict["3"]
    fmt.Println("invalid_query:", invalid_query)

    // Delete an element.
    delete(m_dict, "1")

    // Check if keys are present.
    for i := 1; i < 5; i++ {
        var key string = strconv.Itoa(i)
        value, exists := m_dict[key]
        fmt.Println("key:", key, "| exists:", exists, "| value:", value)
    }
}

type json_example struct {
    f_1 string `json:"f_1"`
    f_2 string `json:"f_2"`
}


func test_json_stuff() {
    f_2 := "qwert"
    obj := json_example {
        f_1: "nil",
        f_2: f_2,
    }

    body, err := json.Marshal(obj)
    if err != nil {
        fmt.Println(err)
    }

    fmt.Println(string((body)[:]))
}


func test_type_interface() {
    // Test out type assertions by themselves
    thing := interface_proxy_to_string().(string)
    fmt.Println("thing: " + thing)
    
    // Check to see if it can get placed into a struct.
    obj := test_return_struct()

    // See if it can directly get placed into a struct.
    fmt.Println(obj)
}

func interface_proxy_to_string () (interface{}) {
    return "kfjdsljf"
}

func test_return_struct() json_example {
    return json_example {
        f_1: interface_proxy_to_string().(string),
        f_2: "thing",
    }
}

func test_struct_equals() {
    x := json_example {
        f_1: "miss",
        f_2: "thing",
    }
    y := json_example {
        f_1: "miss",
        f_2: "thing",
    }

    if x == y {
        fmt.Println("x has equivalent fields to y")
    } else {
        fmt.Println("x does not have equivalent fields to y")
    }
}

func test_slice_length() {
    var sl []string
    fmt.Println(len(sl))
}

func test_map_to_string() {
    // this map is a hashtable implementation
    // map[KeyType]ValueType
    m := make(map[string]string)
    m["red"] = "blue"
    fmt.Printf("%s\n", m)
}

// Declare a struct.
// Note: fields without an explicitly declared name is promoted/embedded
//       fields with an explicitly declared name are not promoted/embedded
type OuterS struct {
    name string
    InnerS
}

// Declare a struct to put within.
type InnerS struct {
    name string
}

/*
// Declare a function for outer struct.
func (o OuterS) getInfo() string {
    return o.name
}
*/

// Declare a function for inner struct.
func (i InnerS) getInfo() string {
    return i.name
}

func test_method_promotion() {
    // Make a struct.
    var i InnerS = InnerS {
        "inner struct",
    }

    // Make a struct with the previous struct in it.
    var o OuterS = OuterS {
        "outer struct",
        i,
    }

    // Call the structs function.
    fmt.Println("i.getinfo(): " + i.getInfo())
    fmt.Println("o.getinfo(): " + o.getInfo())
}

func test_error_equal() {
    // Make an error
    err_1 := fmt.Errorf("an error")

    // Make another error
    err_2 := fmt.Errorf("an error")

    // Make a different error
    err_3 := fmt.Errorf("a different error")

    // Test to see if the errors are equal or not.
    eq_1 := err_1 == err_1
    eq_2 := err_1 == err_3
    eq_3 := err_2 == err_3
    eq_4 := err_1 == err_2

    // Show the results.
    fmt.Printf("eq_1: %b\n", eq_1)
    fmt.Printf("eq_2: %s\n", eq_2)
    fmt.Printf("eq_3: %s\n", eq_3)
    fmt.Printf("eq_4: %s\n", eq_4)
}
