use std::ops::*;
type Get<T> = &'static dyn Fn() -> &'static mut T;
type Set<T> = &'static dyn Fn(T) -> ();

pub struct Property<T:Clone,ListItem> where T:'static {
    closure_get:Get<T>,
    closure_set:Set<T>,
    null:Option<ListItem>
}

impl<T:Clone> Property<T,()> {
    fn get(&self) -> T {
        (self.closure_get)().to_owned()
    }

    fn refr(&self) -> &'static mut T {
        (self.closure_get)()
    }

    fn set(&mut self, value:T){
        (self.closure_set)(value)
    }
}

// region: deref
impl<T:Clone> Deref for Property<T,()> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.refr()
    }
}

impl<T:Clone> DerefMut for Property<T,()> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.refr()
    }
}
// endregion

// region: arithmetic

impl<T:Clone> Add<T> for Property<T,()> where T:Add<Output = T> {
    type Output = T;
    fn add(self, rhs: T) -> Self::Output {
        self.get().to_owned() + rhs
    }
}

impl<T:Clone> Sub<T> for Property<T,()> where T:Sub<Output = T> {
    type Output = T;
    fn sub(self, rhs: T) -> Self::Output {
        self.get().to_owned() - rhs
    }
}

impl<T:Clone> Div<T> for Property<T,()> where T:Div<Output = T> {
    type Output = T;
    fn div(self, rhs: T) -> Self::Output {
        self.get().to_owned() / rhs
    }
}

impl<T:Clone> Mul<T> for Property<T,()> where T:Mul<Output = T> {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        self.get().to_owned() * rhs
    }
}

impl<T:Clone> AddAssign<T> for Property<T,()> where T:Add<Output = T>,T:AddAssign<T> {
    fn add_assign(&mut self, rhs: T) {
        self.set(self.get() + rhs);
    }
}

impl<T:Clone> SubAssign<T> for Property<T,()> where T:Sub<Output = T>,T:SubAssign<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.set(self.get() - rhs);
    }
}

impl<T:Clone> DivAssign<T> for Property<T,()> where T:Div<Output = T>,T:DivAssign<T> {
    fn div_assign(&mut self, rhs: T) {
        self.set(self.get() / rhs);
    }
}

impl<T:Clone> MulAssign<T> for Property<T,()> where T:Mul<Output = T>,T:MulAssign<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.set(self.get() * rhs);
    }
}

impl<T:Clone> Neg for Property<T,()> where T:Neg<Output = T> {
    type Output = T;
    fn neg(self) -> Self::Output {
        -self.get()
    }
}

// endregion

// region: bitwise

impl<T:Clone> Shl<T> for Property<T,()> where T:Shl<Output = T> {
    type Output = T;
    fn shl(self, rhs: T) -> Self::Output {
        self.get() << rhs
    }
}

impl<T:Clone> Shr<T> for Property<T,()> where T:Shr<Output = T> {
    type Output = T;
    fn shr(self, rhs: T) -> Self::Output {
        self.get() >> rhs
    }
}

impl<T:Clone> BitAnd<T> for Property<T,()> where T:BitAnd<Output = T> {
    type Output = T;
    fn bitand(self, rhs: T) -> Self::Output {
        self.get() & rhs
    }
}

impl<T:Clone> BitOr<T> for Property<T,()> where T:BitOr<Output = T> {
    type Output = T;
    fn bitor(self, rhs: T) -> Self::Output {
        self.get() | rhs
    }
}

impl<T:Clone> BitXor<T> for Property<T,()> where T:BitXor<Output = T> {
    type Output = T;
    fn bitxor(self, rhs: T) -> Self::Output {
        self.get() ^ rhs
    }
}

impl<T:Clone> BitAndAssign<T> for Property<T,()> where T:BitAnd<Output = T>,T:BitAndAssign<T> {
    fn bitand_assign(&mut self, rhs: T) {
        self.set(self.get() & rhs);
    }
}

impl<T:Clone> BitOrAssign<T> for Property<T,()> where T:BitOr<Output = T>,T:BitOrAssign<T> {
    fn bitor_assign(&mut self, rhs: T) {
        self.set(self.get() | rhs);
    }
}

impl<T:Clone> BitXorAssign<T> for Property<T,()> where T:BitXor<Output = T>,T:BitXorAssign<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        self.set(self.get() ^ rhs);
    }
}

// endregion

// region: implementations

impl<T:Clone> std::fmt::Display for Property<T,()> where T:std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.get())
    }
}

impl<T:Clone,ListItem> Iterator for Property<T,ListItem> where T:Iterator<Item = ListItem> {
    type Item = ListItem;
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.closure_get)().into_iter())
    }
}

// endregion