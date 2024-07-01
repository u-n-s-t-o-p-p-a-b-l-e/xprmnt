       IDENTIFICATION DIVISION.
       PROGRAM-ID. HelloWorld.

       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT INPUT-FILE ASSIGN TO 'INPUT.DAT'
           ORGANIZATION IS LINE SEQUENTIAL.

       DATA DIVISION.
       FILE SECTION.
       FD  INPUT-FILE.
       01 INPUT-RECORD PIC X(50).

       WORKING-STORAGE SECTION.
       01  WS-NAME       PIC X(50).
       01  WS-EOF        PIC X VALUE 'N'.
       01  WS-NAME-LEN   PIC 9(2) VALUE 0.
       O1  WS-I          PIC 9(2).

       PROCEDURE DIVISION.
       MAIN-PARA.
           OPEN INPUT INPUT-FILE
           PERFORM UNTIL WS-EOF = 'Y'
               READ INPUT-FILE INTO INPUT-RECORD
                   AT END
                       MOVE 'Y' TO WS-EOF
                       NOT AT END

