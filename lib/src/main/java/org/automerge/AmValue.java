package org.automerge;

import java.util.Arrays;
import java.util.Date;

/**
 * Any value in an automerge document
 *
 * <p>
 * This abstract class is the base class for all values in an automerge
 * document. Each nested class represents one of the possible values in the data
 * model. Methods which read an arbitrary value from the document such as
 * {@link Read#get(ObjectId, String)} return an instance of {@link AmValue}.
 * Applications which need to know what specific kind of value is returned
 * should then use instanceof to figure out what value they have.
 */
public abstract class AmValue {

    /** An unsigned integer */
    public static class UInt extends AmValue {
        private long value;

        public long getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "UInt [value=" + value + "]";
        }
    }

    /** A 64bit integer */
    public static class Int extends AmValue {
        private long value;

        public long getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Int [value=" + value + "]";
        }
    }

    /** A Boolean */
    public static class Bool extends AmValue {
        private boolean value;

        public boolean getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Bool [value=" + value + "]";
        }
    }

    /** A byte array */
    public static class Bytes extends AmValue {
        private byte[] value;

        public byte[] getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Bytes [value=" + Arrays.toString(value) + "]";
        }
    }

    /** A string */
    public static class Str extends AmValue {
        private String value;

        public String getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Str [value=" + value + "]";
        }
    }

    /** A 64 bit floating point number */
    public static class F64 extends AmValue {
        private double value;

        public double getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "F64 [value=" + value + "]";
        }
    }

    /** A counter */
    public static class Counter extends AmValue {
        private org.automerge.Counter value;

        public long getValue() {
            return value.getValue();
        }

        @Override
        public String toString() {
            return "Counter [value=" + value.getValue() + "]";
        }
    }

    /** A timestamp */
    public static class Timestamp extends AmValue {
        private Date value;

        public Date getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Timestamp [value=" + value + "]";
        }
    }

    /** A null value */
    public static class Null extends AmValue {

        @Override
        public String toString() {
            return "Null";
        }
    }

    /**
     * An unknown value
     *
     * <p>
     * This is used to represent values which may be added by future versions of
     * automerge.
     */
    public static class Unknown extends AmValue {
        private int typeCode;
        private byte[] value;

        public int getTypeCode() {
            return typeCode;
        }

        public byte[] getValue() {
            return value;
        }

        @Override
        public String toString() {
            return "Unknown [typeCode=" + typeCode + "]";
        }
    }

    /** A map object */
    public static class Map extends AmValue {
        private ObjectId id;

        public ObjectId getId() {
            return id;
        }

        @Override
        public String toString() {
            return "Map [id=" + id + "]";
        }
    }

    /** A list object */
    public static class List extends AmValue {
        private ObjectId id;

        public ObjectId getId() {
            return id;
        }

        @Override
        public String toString() {
            return "List [id=" + id + "]";
        }
    }

    /** A text object */
    public static class Text extends AmValue {
        private ObjectId id;

        public ObjectId getId() {
            return id;
        }

        @Override
        public String toString() {
            return "Text [id=" + id + "]";
        }
    }
}
