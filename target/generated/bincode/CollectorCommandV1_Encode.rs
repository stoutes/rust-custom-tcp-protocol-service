impl :: bincode :: Encode for CollectorCommandV1
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        match self
        {
            Self ::SubmitData
            { collector_id, total_memory, used_memory, average_cpu_usage }
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (0u32), encoder) ?
                ; :: bincode :: Encode :: encode(collector_id, encoder) ?; ::
                bincode :: Encode :: encode(total_memory, encoder) ?; ::
                bincode :: Encode :: encode(used_memory, encoder) ?; ::
                bincode :: Encode :: encode(average_cpu_usage, encoder) ?;
                core :: result :: Result :: Ok(())
            }, Self ::RequestWork(field_0)
            =>{
                < u32 as :: bincode :: Encode >:: encode(& (1u32), encoder) ?
                ; :: bincode :: Encode :: encode(field_0, encoder) ?; core ::
                result :: Result :: Ok(())
            },
        }
    }
}