package org.automerge;

import java.util.Date;

/**
 * Any non-object value to be added to a document
 *
 * <p>
 * Occasionally you may need to be generic over the kind of new value to insert
 * into a document (for example in
 * {@link Transaction#set(ObjectId, String, NewValue)}). This class encapsulates
 * all the possible non-object values one can create. You should use one of the
 * static methods on NewValue to create an instance.
 */
public abstract class NewValue {
    protected abstract void set(Transaction tx, ObjectId obj, String key);

    protected abstract void set(Transaction tx, ObjectId obj, long idx);

    protected abstract void insert(Transaction tx, ObjectId obj, long idx);

    protected abstract void mark(Transaction tx, ObjectId obj, long start, long end, String markName,
            ExpandMark expand);

    /**
     * Create a new unsigned integer value
     *
     * @param value
     *            the positive integer value
     * @return a new unsigned integer value
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public static NewValue uint(long value) {
        return new UInt(value);
    }

    /**
     * A new integer value
     *
     * @param value
     *            the integer value
     * @return a new integer value
     */
    public static NewValue integer(long value) {
        return new Int(value);
    }

    /**
     * A new floating point value
     *
     * @param value
     *            the floating point value
     * @return a new floating point value
     */
    public static NewValue f64(double value) {
        return new NewValue.F64(value);
    }

    /**
     * A new boolean value
     *
     * @param value
     *            the boolean value
     * @return a new boolean value
     */
    public static NewValue bool(boolean value) {
        return new NewValue.Bool(value);
    }

    /**
     * A new string value
     *
     * @param value
     *            the string value
     * @return a new string value
     */
    public static NewValue str(String value) {
        return new NewValue.Str(value);
    }

    /**
     * A new byte array value
     *
     * @param value
     *            the byte array value
     * @return a new byte array value
     */
    public static NewValue bytes(byte[] value) {
        return new NewValue.Bytes(value);
    }

    /**
     * A new counter value
     *
     * @param value
     *            the initial value of the counter
     * @return a new counter value
     */
    public static NewValue counter(long value) {
        return new NewValue.Counter(value);
    }

    /**
     * A new timestamp value
     *
     * @param value
     *            the value of the timestamp
     * @return a new timestamp value
     */
    public static NewValue timestamp(Date value) {
        return new NewValue.Timestamp(value);
    }

    /** The null value */
    public static NewValue NULL = new NewValue.Null();

    /** A new unsigned integer value */
    public static class UInt extends NewValue {
        private long value;

        protected UInt(long value) {
            if (value < 0) {
                throw new IllegalArgumentException("UInt must be positive");
            }
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.setUint(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.setUint(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insertUint(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.markUint(obj, start, end, markName, value, expand);
        }
    }

    /** A new integer value */
    public static class Int extends NewValue {
        private long value;

        protected Int(long value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }

    /** A new floating point value */
    public static class F64 extends NewValue {
        private double value;

        protected F64(double value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }

    /** A new string value */
    public static class Str extends NewValue {
        private java.lang.String value;

        protected Str(java.lang.String value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }

    /** A new boolean value */
    public static class Bool extends NewValue {
        private boolean value;

        protected Bool(boolean value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }

    /** A new null value */
    public static class Null extends NewValue {
        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.setNull(obj, key);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.setNull(obj, idx);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insertNull(obj, idx);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.markNull(obj, start, end, markName, expand);
        }
    }

    /** A new byte array value */
    public static class Bytes extends NewValue {
        private byte[] value;

        protected Bytes(byte[] value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }

    /** A new counter value */
    public static class Counter extends NewValue {
        private long value;

        protected Counter(long value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, new org.automerge.Counter(value));
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, new org.automerge.Counter(value));
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, new org.automerge.Counter(value));
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, new org.automerge.Counter(value), expand);
        }
    }

    /** A new timestamp value */
    public static class Timestamp extends NewValue {
        private Date value;

        protected Timestamp(Date value) {
            this.value = value;
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, String key) {
            tx.set(obj, key, value);
        }

        @Override
        protected void set(Transaction tx, ObjectId obj, long idx) {
            tx.set(obj, idx, value);
        }

        @Override
        protected void insert(Transaction tx, ObjectId obj, long idx) {
            tx.insert(obj, idx, value);
        }

        @Override
        protected void mark(Transaction tx, ObjectId obj, long start, long end, String markName, ExpandMark expand) {
            tx.mark(obj, start, end, markName, value, expand);
        }
    }
}
