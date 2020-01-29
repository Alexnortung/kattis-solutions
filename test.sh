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
        # compare both files
        echo "$OUTVALUE" | while read comparefile1 <"$FILENAME.out" && read comparefile2
        do
            if [ "$comparefile1" != "$comparefile2" ] 
            then
                printf "❌\ngot: %s expected: %s\n" "$comparefile2" "$comparefile1"
                echo "failed at $FILENAME files"
                echo "Showing diff"
                echo `echo "$OUTVALUE" | diff "$FILENAME.out" -`
                exit 1
            fi
            # is same
        done
        # print check when file is completed
        if [ "$?" == 1 ]; then
            exit 1
        else
            printf "✅ "
        fi
    done
    if [ "$?" == "0" ]
    then
        printf "\nAll tests passed for $TYPE\n"
    fi
    echo "-------------"
}

function compileRunRust() {
    echo "compiling rust file to binary"
    BINARYPATH="$BINDIR/$NAME-rust"
    rustc "$file" -o "$BINARYPATH"
    testrun "$BINARYPATH" "rust"
}

function runPython() {
    testrun "python $PROBLEMDIR/src/$NAME.py" "python"
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
        if [ "$extension" == "rs" ] 
        then
            compileRunRust
        elif [ "$extension" == "py" ] 
        then
            #TYPES=("${TYPES[@]}" "python")
            runPython
        fi
        #printf "%s " "${TYPES[@]}"
    done
elif [ "$TYPE" == "rust" ]; then
    compileRunRust
elif [ "$TYPE" == "python" ]; then
    runPython
fi
