import msgpack


    def GetAttribute(self, key):
        py_result = self.inst.get().GetAttributePy(<libcpp_string> key)
        return <object>py_result
        
        
    def SetAttribute(self, key, value):
        t = type(value)
        if t == str:
            self.inst.get().SetAttribute(<libcpp_string> key, <libcpp_string> value)
        elif t == float:
            self.inst.get().SetAttribute(<libcpp_string> key, <float> value)
        elif t == int:
            self.inst.get().SetAttribute(<libcpp_string> key, <int> value)
        # special trick as t == bool does not work due to name collision between cython and C
        elif isinstance(value, (int)):
            self.inst.get().SetAttribute(<libcpp_string> key, <bool> value)
        else:
            raise Exception("Unsupported Value Type")

    def dumps(self):
        m=[]
        do_pack_rest = False
        score = self.inst.get().GetScore()
        if score != 0:
            m.append(score)
            do_pack_rest = True
        end = self.inst.get().GetEnd()
        if end != 0 or do_pack_rest:
            m.append(end)
            do_pack_rest = True
        start = self.inst.get().GetStart()
        if start != 0 or do_pack_rest:
            m.append(start)
            do_pack_rest = True
        matchedstring = self.inst.get().GetMatchedString()
        if len(matchedstring) != 0 or do_pack_rest:
            m.append(matchedstring)
            do_pack_rest = True
        rawvalue = self.inst.get().GetRawValueAsString()
        if len(rawvalue) != 0 or do_pack_rest:
            m.append(rawvalue)
        m.reverse()
        return msgpack.dumps(m)

    def __SetRawValue(self, str):
         self.inst.get().SetRawValue(<libcpp_string> str)

    @staticmethod
    def loads(serialized_match):
        m=Match()
        unserialized = msgpack.loads(serialized_match)
        number_of_fields = len(unserialized)
        if number_of_fields > 0:
            m.__SetRawValue(unserialized[0])
            if number_of_fields > 1:
                m.SetMatchedString(unserialized[1])
                if number_of_fields > 2:
                    m.SetStart(unserialized[2])
                    if number_of_fields > 3:
                         m.SetEnd(unserialized[3])
                         if number_of_fields > 4:
                             m.SetScore(unserialized[4])

        return m

