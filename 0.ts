// Remove the 'kind' property

type RemoveKindField<Type> = {
  [Property in keyof Type as Exclude<Property, "toRemove">]: Type[Property];
};

interface Circle {
  radius: number;
  toRemove: void;
}

type KindlessCircle = RemoveKindField<Circle>;

const a: KindlessCircle = {
  radius: 3,
};

type ExtractPII<Type> = {
  [Property in keyof Type]: Type[Property] extends { pii: true } ? true : false;
};

type DBFields = {
  id: { format: "incrementing" };
  name: { type: string; pii: true };
};

type ObjectsNeedingGDPRDeletion = ExtractPII<DBFields>;

type KeysExcludingSpecificType<Type, TypeToExclude> = {
  [Property in keyof Type]: Type[Property] extends TypeToExclude
    ? never
    : Property;
}[keyof Type];

type ObjectExcludingSpecificTypes<Type, TypeToExclude> = Pick<
  Type,
  KeysExcludingSpecificType<Type, TypeToExclude>
>;

type XYZ = { x: void; y: void; z: number };
type T3a = Pick<XYZ, KeysExcludingSpecificType<XYZ, void>>; // { x: number; z: number; }

type ObjectExcludingVoid<Type> = ObjectExcludingSpecificTypes<Type, void>;

type P = ObjectExcludingVoid<{ a: number; b: void }>;

type RemoveVoidField<Type> = {
  [Property in keyof Type as Exclude<
    Property,
    Type[Property] extends void ? Property : never
  >]: Type[Property];
};

type OmitVoid<T> = { [K in keyof T as T[K] extends void ? never : K]: T[K] };

class Test<T, C, E> {
  add(params: OmitVoid<{ a: T; b: C; c: E }>) {
    const { a, b } = params as { a: T; b: C; c: E };
    console.log(b);
  }
}

const test = new Test<void, void, string>();

test.add({ c: "3" });

interface Square {
  radius: number;
  toRemove: void;
}

type PP = OmitVoid<Square>;

const cc: PP = {
  radius: 3,
};
