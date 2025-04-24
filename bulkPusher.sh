
# Bulk Pusher script.
# Date: 24 April 2025
# Oliver Bonham-Carter, obonhamcarter@allegheny.edu
# This script uses the File, `dirNames.txt`, to locate repositories to push
# The current date is printed in the commit message of the submit
# A file, "0_thisLastPush.txt" is created to state when the last bulk push was completed.


NOW=`date`
printf "Current date and time in Linux is: $NOW"

date > 0_thisLastPush.txt

pwd > mydir
for z in `cat mydir`; do cd $z; done
for DIRNAME in $(cat dirNames.txt);  # This file should contain the names of the directories to be pushed
do
    cd $DIRNAME
    echo Checking: $DIRNAME
     echo git add -A
     echo git commit -m "Grade update: $NOW"
     echo git push
    cd $z/
done

rm mydir
