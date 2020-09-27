// Roundtrip Pong.

use cyclonedds_rs::dds_reliability_kind;
use cyclonedds_rs::{
    dds_attach_t, dds_duration_t, DdsPublisher, DdsQos, DdsReader, DdsSubscriber, DdsTopic,
    DdsWriter, DdsWaitset, DdsReadCondition, dds_api::StateMask, dds_api::State, dds_triggered, take, DdsLoanedData,
};

use roundtrip_data::RoundTripModule::DataType;
use roundtrip_data::*;
fn main() {
    println!("Roundtrip Pong!");
    let wait_timeout: dds_duration_t = std::i64::MAX;
    if let Ok(participant) = cyclonedds_rs::DdsParticipant::create(None, None, None) {
        if let Ok(topic) = DdsTopic::<RoundTripModule::DataType>::create(&participant, "RoundTrip", None, None) {
            let mut qos = cyclonedds_rs::DdsQos::create().unwrap();
            qos.set_partition(&std::ffi::CString::new("pong").unwrap());

            let publisher = DdsPublisher::create(&participant, Some(&qos), None)
                .expect("Unable to create publisher");

            // A Dds data writer is created on the publisher and topic
            let mut writer_qos = DdsQos::create().expect("Unable to create qos");
            let duration = std::time::Duration::from_secs(10);
            writer_qos.set_reliability(
                dds_reliability_kind::DDS_RELIABILITY_RELIABLE,
                duration.as_nanos() as i64,
            );
            writer_qos.set_writer_data_lifecycle(false);

            let mut writer =
                DdsWriter::<RoundTripModule::DataType>::create(&publisher, &topic, Some(&writer_qos), None).unwrap();

            // A dds subscriber is created on the domain participant
            let mut qos = DdsQos::create().unwrap();
            qos.set_partition(&std::ffi::CString::new("ping").unwrap());

            let subscriber = DdsSubscriber::create(&participant, Some(&qos), None).unwrap();

            // a data reader is created on the subscriber and topic
            let mut qos = DdsQos::create().unwrap();
            qos.set_reliability(
                dds_reliability_kind::DDS_RELIABILITY_RELIABLE,
                duration.as_nanos() as i64,
            );

            let listener = cyclonedds_rs::DdsListener::new()
            .on_data_available(move|entity| {

                //println!("Data available");
                
                    if let Ok(data) = unsafe {cyclonedds_rs::take::<RoundTripModule::DataType>(&entity)} {
                        let data = data.as_slice();
                        println!("Received data len:{}",data.len());

                        for d in data {
                            println!("Payload len:{}",d.payload._length);
                            let ret = writer.write(&d);
                        }
                    } else {
                        panic!("take failed");
                    }

            })
            .hook();

            let mut reader = DdsReader::create(&subscriber, &topic, Some(&qos), Some(listener)).unwrap();

            let mut waitset = DdsWaitset::<isize>::create(&participant).expect("Unable to create waitset");
            
           // let mut mask = StateMask::none();
           // mask.set(State::DdsAnyState);

            //let read_condition = reader.create_readcondition(mask).expect("Unable to create readcondition");
            //let i = 0;
            //let j = 0;
            //waitset.attach(&read_condition,&i).expect("Failed to attach waitset");
            
            ////waitset.attach(&waitset,&j).expect("Failed to attack waitset waitset");

            while dds_triggered(&waitset).is_ok() {
                let mut xs = Vec::new();
                xs.reserve(10);

                if let Ok(xs) = waitset.wait(&mut xs,wait_timeout) {
                    if xs.len() == 0 {
                        
                    } else {
                        //println!("Wait returned something");
                    }

                } else {
                    println!("wait returned error");
                }

            }

            loop {
                panic!("Loop done");
            }

        } else {
            panic!("Unable to create topic RoundTrip");
        }
    } else {
        panic!("Unable to create participant");
    }
}
