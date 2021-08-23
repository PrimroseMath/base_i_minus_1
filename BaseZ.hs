data ComplexInt = !Integer :+ !Integer deriving (Eq,Ord,Show) -- built in Complex type wants RealFloats

instance Num ComplexInt where
    (a :+ b) + (c :+ d) = (a + c) :+ (b + d)
    (a :+ b) * (c :+ d) = (a*c - b*d) :+ (a*d + b*c)
    abs (a :+ b) = fromInteger (a*a + b*b) -- actually abs squared, but haskell num class is annoying
    fromInteger n = n :+ 0
    signum (a :+ _) = fromInteger (signum a) -- again, num class is annoying
    negate (a :+ b) = (negate a) :+ (negate b)


base :: ComplexInt
base = (-1) :+ 1

cdiv (a :+ b) z2@(c :+ d) = ((a*c + b*d) `div` norm2) :+ ((b*c - a*d) `div` norm2)
    where
    norm2 = c*c + d*d

convertFromBaseZ :: String -> ComplexInt
convertFromBaseZ s = cfbz s (0 :+ 0)
    where
    cfbz :: String -> ComplexInt -> ComplexInt
    cfbz [] sofar = sofar
    cfbz ('1':s) sofar = cfbz s (sofar * base + 1)
    cfbz ('0':s) sofar = cfbz s (sofar*base)
    cfbz (_:s) sofar = cfbz s sofar -- Just ignore other characters, I'm too lazy to do error checking

convertToBaseZ :: ComplexInt -> String
convertToBaseZ (0 :+ 0) = "0"
convertToBaseZ z = reverse (ctbz z)
    where
    ctbz (0 :+ 0)  = ""
    ctbz z@(a :+ b) = (if even ( a- b) then '0' else '1') : (ctbz (z' `cdiv` base))
        where
        z' = if even (a - b) then z else z - 1