def test_from_import():
    from fastformat import hello as fastformat_hello
    from fastformat.datatypes import hello as datatypes_hello

    assert fastformat_hello() == "hello fastformat"
    assert datatypes_hello() == "hello datatypes"

def test_direct_import():
    import fastformat
    from fastformat import datatypes

    assert fastformat.hello() == "hello fastformat"
    assert datatypes.hello() == "hello datatypes"

def main():
    test_from_import()
    test_direct_import()

if __name__ == "__main__":
    main()
