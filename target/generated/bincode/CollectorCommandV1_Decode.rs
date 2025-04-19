impl < __Context > :: bincode :: Decode < __Context > for CollectorCommandV1
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result ::
            Ok(Self ::SubmitData
            {
                collector_id : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?, total_memory : :: bincode :: Decode ::< __D
                :: Context >:: decode(decoder) ?, used_memory : :: bincode ::
                Decode ::< __D :: Context >:: decode(decoder) ?,
                average_cpu_usage : :: bincode :: Decode ::< __D :: Context
                >:: decode(decoder) ?,
            }), 1u32 =>core :: result :: Result ::
            Ok(Self ::RequestWork
            {
                0 : :: bincode :: Decode ::< __D :: Context >::
                decode(decoder) ?,
            }), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "CollectorCommandV1", allowed :
                &:: bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 1 }
            })
        }
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for CollectorCommandV1
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        let variant_index = < u32 as :: bincode :: Decode ::< __D :: Context
        >>:: decode(decoder) ?; match variant_index
        {
            0u32 =>core :: result :: Result ::
            Ok(Self ::SubmitData
            {
                collector_id : :: bincode :: BorrowDecode ::< __D :: Context
                >:: borrow_decode(decoder) ?, total_memory : :: bincode ::
                BorrowDecode ::< __D :: Context >:: borrow_decode(decoder) ?,
                used_memory : :: bincode :: BorrowDecode ::< __D :: Context
                >:: borrow_decode(decoder) ?, average_cpu_usage : :: bincode
                :: BorrowDecode ::< __D :: Context >:: borrow_decode(decoder)
                ?,
            }), 1u32 =>core :: result :: Result ::
            Ok(Self ::RequestWork
            {
                0 : :: bincode :: BorrowDecode ::< __D :: Context >::
                borrow_decode(decoder) ?,
            }), variant =>core :: result :: Result ::
            Err(:: bincode :: error :: DecodeError :: UnexpectedVariant
            {
                found : variant, type_name : "CollectorCommandV1", allowed :
                &:: bincode :: error :: AllowedEnumVariants :: Range
                { min: 0, max: 1 }
            })
        }
    }
}