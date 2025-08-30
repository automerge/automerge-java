package org.automerge;

import java.util.ArrayList;

class PathBuilder {
    private ArrayList<PathElement> elements = new ArrayList<>();

    public static ArrayList<PathElement> empty() {
        return new ArrayList<>();
    }

    public static PathBuilder root(String key) {
        PathBuilder pb = new PathBuilder();
        pb.elements.add(new PathElement(ObjectId.ROOT, new Prop.Key(key)));
        return pb;
    }

    public PathBuilder key(ObjectId obj, String key) {
        elements.add(new PathElement(obj, new Prop.Key(key)));
        return this;
    }

    public PathBuilder index(ObjectId obj, long idx) {
        elements.add(new PathElement(obj, new Prop.Index(idx)));
        return this;
    }

    public ArrayList<PathElement> build() {
        return elements;
    }
}
