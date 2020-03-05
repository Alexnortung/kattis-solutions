# should be called from repo root directory

# get the name as an argurment
# get filetype as an argurment (optionally)

# check if the folder with that name exists
# check which file types are in the directory or check if the given filetype are in the directory

NAME=$1
TYPE=$2

mkdir -p bin

if [ "$TYPE" == "" ]; then
    TYPE="all"
fi

if [ "$NAME" == "" ]; then
    echo "no name given"
    exit 1
fi

PROBLEMDIR="problems/$NAME"
BINDIR="bin"

if [ ! -d "$PROBLEMDIR" ]; then
    echo "directory does not exists"
    exit 1
fi

function testrun() {
    BINARYPATH=$1 # or command to run (e.g. python: python filename)
    TYPE=$2
    echo "running test for $TYPE"
    find "$PROBLEMDIR/testfiles" -type f -regex ".*\.in" -print0 | while read -d $'\0' file
    do
        FILENAME="${file%.*}"
        OUTVALUE=`cat "$file" | $BINARYPATH` # value after program has run
        CORRECTOUTVALUE=`cat "$FILENAME".ans`
        echo "$OUTVALUE" | diff -q --strip-trailing-cr "$FILENAME.ans" -
        # check if the diff was successful or not
        if [ "$?" != "0" ]
        then
            printf "❌\ngot: \n%s \nexpected: \n%s\n" "$OUTVALUE" "$CORRECTOUTVALUE"
            echo "failed at $FILENAME files"
            echo "Showing diff"
            echo "$OUTVALUE" | diff -y --strip-trailing-cr "$FILENAME.ans" -
            exit 1
        fi
        # print check when file is completed
        printf "✅ "
    done
    if [ "$?" == "0" ]
    then
        printf "\nAll tests passed for $TYPE\n"
    fi
    echo "-------------"
}

function compileRunRust() {
    BINARYPATH="$BINDIR/$NAME-rust"
    COMPILECOMMAND="rustc $PROBLEMDIR/src/$NAME.rs -o $BINARYPATH"
    compileRun "rust" "$COMPILECOMMAND"
}

function compileRunCPP() {
    BINARYPATH="$BINDIR/$NAME-cpp"
    COMPILECOMMAND="g++ $PROBLEMDIR/src/$NAME.cpp -o $BINARYPATH"
    compileRun "cpp" "$COMPILECOMMAND"
}

function runPython() {
    testrun "python $PROBLEMDIR/src/$NAME.py" "python"
}

function runPHP() {
    testrun "php $PROBLEMDIR/src/$NAME.php" "php"
}

function compileRun() {
    TYPE=$1
    COMPILECOMMAND=$2
    BINARYPATH="$BINDIR/$NAME-$TYPE"
    echo "compiling $TYPE file to binary"
    `$COMPILECOMMAND`
    testrun "$BINARYPATH" "$TYPE"
}

if [ "$TYPE" == "all" ]; then
    # find which filytypes exists in the directory.
    find "$PROBLEMDIR/src" -type f -print0 | while read -d $'\0' file
    do
        extension="${file##*.}"
        if [ "$extension" == "rs" ]; then
            compileRunRust
        elif [ "$extension" == "py" ]; then
            runPython
        elif [ "$extension" == "cpp" ]; then
            compileRunCPP
        elif [ "$extension" == "php" ]; then
            runPHP
        fi
        #printf "%s " "${TYPES[@]}"
    done
elif [ "$TYPE" == "rust" ]; then
    compileRunRust
elif [ "$TYPE" == "python" ]; then
    runPython
elif [ "$TYPE" == "cpp" ]; then
    compileRunCPP
elif [ "$TYPE" == "php" ]; then
    runPHP
fi
