package org.automerge;

/** The two kinds of property in a document */
public abstract class Prop {
    /** A key in a map */
    public static final class Key extends Prop {
        public final String key;

        public Key(String key) {
            this.key = key;
        }

        /**
         * The key
         *
         * @return The key
         */
        public String getValue() {
            return key;
        }

        @Override
        public int hashCode() {
            final int prime = 31;
            int result = 1;
            result = prime * result + ((key == null) ? 0 : key.hashCode());
            return result;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null)
                return false;
            if (getClass() != obj.getClass())
                return false;
            Key other = (Key) obj;
            if (key == null) {
                if (other.key != null)
                    return false;
            } else if (!key.equals(other.key))
                return false;
            return true;
        }
    }

    /** An index in a list or text */
    public static final class Index extends Prop {
        public final long index;

        public Index(long index) {
            this.index = index;
        }

        /**
         * The index
         *
         * @return The index
         */
        public long getValue() {
            return index;
        }

        @Override
        public int hashCode() {
            final int prime = 31;
            int result = 1;
            result = prime * result + (int) (index ^ (index >>> 32));
            return result;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null)
                return false;
            if (getClass() != obj.getClass())
                return false;
            Index other = (Index) obj;
            if (index != other.index)
                return false;
            return true;
        }
    }
}
