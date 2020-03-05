

NAME=$1

BASEDIR="problems/$NAME"
TESTDIR="$BASEDIR/testfiles"
mkdir -p "$BASEDIR/src"
mkdir -p "$TESTDIR"

SAMPLESURL="https://open.kattis.com/problems/$NAME/file/statement/samples.zip"
STATUSCODE="$(curl -LI "${SAMPLESURL}" -o /dev/null -w '%{http_code}' -s)"

echo "code: $STATUSCODE"
#echo $SAMPLESURL

if [ "$STATUSCODE" == "200" ]; then
    echo "Found sample files, downloading..."
    OUTZIP="$TESTDIR/samples.zip"
    curl $SAMPLESURL --output $OUTZIP
    unzip $OUTZIP -d $TESTDIR
    rm $OUTZIP
else
    echo "Problem sample files was not found"
fi