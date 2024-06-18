use core::{fmt, marker::PhantomData};

use super::{DesIntoWithConfig, DeserializeInto, DeserializerConfig};

pub struct VecDeserializer<W>(PhantomData<W>);

impl<W, T> DeserializeInto<Vec<T>> for VecDeserializer<W>
where
    W: DeserializeInto<T>,
{
    #[inline]
    fn deserialize_into<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
        config: &DeserializerConfig,
    ) -> Result<Vec<T>, D::Error> {
        struct Visitor<'c, W, T>(&'c DeserializerConfig, PhantomData<(W, T)>);

        impl<'c, 'de, W, T> serde::de::Visitor<'de> for Visitor<'c, W, T>
        where
            W: DeserializeInto<T>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let capacity = super::size_hint::cautious::<T>(seq.size_hint());
                let mut values = Vec::<T>::with_capacity(capacity);

                while let Some(value) =
                    seq.next_element_seed(DesIntoWithConfig::<W, T>::new(self.0))?
                {
                    values.push(value);
                }

                Ok(values)
            }
        }

        deserializer.deserialize_seq(Visitor::<W, T>(config, PhantomData))
    }
}