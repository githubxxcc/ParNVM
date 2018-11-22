//************************************************
//This module has concrete implementation for the 
//TRef trait.
//
//Types: 
//- WarehouseRef
//- DistrictRef
//- XXXXRef
//
//Impl: 
//- BucketDeleteRef
//- TableRef
//- BucketPushRef
//************************************************

/* FIXME: this should not be needed with GAT implemented */
/* FIXME: this should be much shorter with procedures macros, 
 * but copy-paste is easier for now... */


use super::entry::*;
use super::table::*;
use super::tpcc_tables::*;
use pnvm_lib:: {
    tcore::*,
    tbox::*,
    txn::*,
};
use std::{
    any::Any,
    sync::Arc,
};

#[cfg(not(any(feature = "pmem", feature = "disk")))]
use core::alloc::Layout;

#[cfg(any(feature = "pmem", feature = "disk"))]
use pnvm_sys::{
    Layout
};



#[derive(Clone , Debug)]
pub struct WarehouseRef  {
    inner_ : Arc<Row<Warehouse, i32>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<Warehouse>>,
    ops_ : Operation,
}


#[derive(Clone , Debug)]
pub struct DistrictRef  {
    inner_ : Arc<Row<District, (i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<District>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct CustomerRef  {
    inner_ : Arc<Row<Customer, (i32, i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<Customer>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct NewOrderRef  {
    inner_ : Arc<Row<NewOrder, (i32, i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<NewOrder>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct OrderRef  {
    inner_ : Arc<Row<Order, (i32, i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<Order>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct OrderLineRef  {
    inner_ : Arc<Row<OrderLine, (i32, i32, i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<OrderLine>>,
    ops_ : Operation,
}


#[derive(Clone , Debug)]
pub struct ItemRef  {
    inner_ : Arc<Row<Item, i32>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<Item>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct StockRef  {
    inner_ : Arc<Row<Stock, (i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<Stock>>,
    ops_ : Operation,
}

#[derive(Clone , Debug)]
pub struct HistoryRef  {
    inner_ : Arc<Row<History, (i32, i32)>>,
    bucket_idx_: Option<usize>,
    table_ref_: Option<Arc<Tables>>,
    //txn_info_ : Option<Arc<TxnInfo>>,
    data_ : Option<Box<History>>,
    ops_ : Operation,
}

impl  TRef for WarehouseRef {
   // fn install(&self, id: Tid) {
   //     match self.table_ref_ {
   //         Some(ref table) => {
   //         },
   //         None => {
   //         }
   //     }
   // }


    fn install(&self, id: Tid) {
        match self.ops_ {
            Operation::Push => {
                let table = self.table_ref_.as_ref().expect("Warehouse_ref: no table ref for push");
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.warehouse.get_bucket(bucket_idx).set_version(row.get_version());
                table.warehouse.get_bucket(bucket_idx).push(row);
            },
            Operation::RWrite => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            },
            _ =>  panic!("Unknown Operations")
        }
    }
    

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }

    //TODO:
    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }

    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<Warehouse>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("warehosuref::write value should be Box<Warehouse>")
        }
    }

   // fn replace_filed(&mut self, vals: &[(usize, &Any)], val_cnt: usize) {
   //     //TODO:
   //     // 1. store the vals
   // }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.warehouse.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.warehouse.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }

    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }

    fn get_name(&self) -> String {
        String::from("warehouse")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}



impl  TRef for DistrictRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.district.get_bucket(bucket_idx).set_version(row.get_version());
                table.district.get_bucket(bucket_idx).push(row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }

    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }
    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }

    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<District>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("DistrictRef::write value should be Box<Warehouse>")
        }
    }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.district.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.district.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }

    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()

    }
    fn get_name(&self) -> String {
        String::from("district")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}



impl  TRef for CustomerRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.customer.get_bucket(bucket_idx).set_version(row.get_version());
                table.customer.get_bucket(bucket_idx).push(row.clone());
                table.customer.update_sec_index(&row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<Customer>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("DistrictRef::write value should be Box<Warehouse>")
        }
    }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.customer.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.customer.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }


    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }

    fn get_name(&self) -> String {
        String::from("customer")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}


impl  TRef for NewOrderRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                match self.ops_ {
                    Operation::Push => {
                        let row = self.inner_.clone();
                        let bucket_idx = self.bucket_idx_.expect("no bucketidx");
                        table.neworder.get_bucket(bucket_idx).set_version(row.get_version());
                        table.neworder.get_bucket(bucket_idx).push(row.clone());
                        table.neworder.update_wd_index(&row);
                    }, 
                    Operation::Delete => {
                        let row = self.inner_.clone();
                        let bucket_idx = self.bucket_idx_.expect("no bucket indx");
                        table.neworder.get_bucket(bucket_idx).set_version(row.get_version());
                        //FIXME: hack so double delete allowed
                        if table.neworder.delete_index(&row) {
                            table.neworder.get_bucket(bucket_idx).delete(row);
                        }
                    }
                    _ => panic!("NewOrderRef::install: RWrite has table ref")
                }
            },
            None => {
                self.inner_.install(self.data_.as_ref().expect("no data"), id);
            }
        }
    }


    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                match self.ops_ {
                    Operation::Delete => {
                        //Lock both the deleted entry and the bucket
                        table.neworder.get_bucket(self.bucket_idx_.unwrap()).lock(tid) && self.inner_.lock(tid)
                    },
                    Operation::Push => {
                        table.neworder.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
                    },
                    _ => panic!("NewOrderRef::lock : RWrite has table ref")
                }
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                match self.ops_ {
                    Operation::Delete => {
                        //Lock both the deleted entry and the bucket
                        self.inner_.unlock();
                        table.neworder.get_bucket(self.bucket_idx_.unwrap()).unlock();
                    },
                    Operation::Push => {
                        table.neworder.get_bucket(self.bucket_idx_.unwrap()).unlock();
                    },
                    _ => panic!("NewOrderRef::unlock : RWrite has table ref")
                }
            }
        }
    }
    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }
    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    
    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }
    


    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }
    
    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }


    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<NewOrder>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("DistrictRef::write value should be Box<Warehouse>")
        }
    }


    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }


    fn get_name(&self) -> String {
        String::from("neworder")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}

impl  TRef for OrderRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.order.get_bucket(bucket_idx).set_version(row.get_version());
                table.order.get_bucket(bucket_idx).push(row.clone());
                table.order.update_cus_index(&row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.order.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.order.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<Order>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("DistrictRef::write value should be Box<Warehouse>")
        }
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }


    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }

    fn get_name(&self) -> String {
        String::from("order")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}

impl  TRef for OrderLineRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.orderline.get_bucket(bucket_idx).set_version(row.get_version());
                table.orderline.get_bucket(bucket_idx).push(row.clone());
                 info!("[{:?}] [ORDERLINE-INDEX] Inserting for {:?}", id, row);
                table.orderline.update_order_index(&row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.orderline.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.orderline.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true //FIXME: what if deleted
        }
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }


    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }
    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<OrderLine>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("DistrictRef::write value should be Box<Warehouse>")
        }
    }

    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }
    fn get_name(&self) -> String {
        String::from("orderline")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}

impl  TRef for ItemRef  {
    fn install(&self, id: Tid) {
        panic!("Item is read only");
    }


    fn lock(&self, tid: Tid) -> bool {
        panic!("Item is read only");
    }

    fn unlock(&self) {
        panic!("Item is read only");
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn write(&mut self, val: Box<Any>) {
        panic!("Item is read only")
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }


    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        //Noop for Item
        //self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }

    fn get_name(&self) -> String {
        String::from("item")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}

impl  TRef for HistoryRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.history.get_bucket(bucket_idx).set_version(row.get_version());
                table.history.get_bucket(bucket_idx).push(row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }

    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.history.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.history.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }

    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }
    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }


    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<History>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("Stock::write value should be Box<Warehouse>")
        }
    }

    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }
    fn get_name(&self) -> String {
        String::from("history")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}
impl  TRef for StockRef  {
    fn install(&self, id: Tid) {
        match self.table_ref_ {
            Some(ref table) => {
                let row = self.inner_.clone();
                let bucket_idx = self.bucket_idx_.unwrap();
                table.stock.get_bucket(bucket_idx).set_version(row.get_version());
                table.stock.get_bucket(bucket_idx).push(row);
            },
            None => {
                self.inner_.install(self.data_.as_ref().unwrap(), id);
            }
        }
    }


    fn lock(&self, tid: Tid) -> bool {
        match self.table_ref_ {
            None => self.inner_.lock(tid),
            Some(ref table) => {
                table.stock.get_bucket(self.bucket_idx_.unwrap()).lock(tid)
            }
        }
    }

    fn unlock(&self) {
        match self.table_ref_ {
            None => self.inner_.unlock(),
            Some(ref table) => {
                table.stock.get_bucket(self.bucket_idx_.unwrap()).unlock();
            }
        }
    }
    fn check(&self, vers: u32, tid: u32) -> bool {
        if self.table_ref_.is_none() {
            self.inner_.check(vers, tid)
        } else {
            true
        }
    }
    fn box_clone(&self) -> Box<dyn TRef> {
        Box::new(self.clone())
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_addr(&self) -> *mut u8 {
        self.inner_.get_pmem_addr() as *mut u8
    }
    fn get_ptr(&self) -> *mut u8 {
        self.inner_.get_ptr()
    }

    fn get_layout(&self) -> Layout {
        self.inner_.get_layout()
    }

    #[cfg(any(feature = "pmem", feature = "disk"))]
    fn get_pmem_field_addr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_pmem_field_addr(field_idx) as *mut u8
    }

    //TODO:
    fn get_field_ptr(&self, field_idx: usize) -> *mut u8 {
        self.inner_.get_field_ptr(field_idx)
    }

    fn get_field_size(&self, field_idx: usize) -> usize {
        self.inner_.get_field_size(field_idx)
    }

    fn get_id(&self) -> &ObjectId {
        self.inner_.get_id()
    }

    fn get_version(&self) -> u32 {
        self.inner_.get_version()
    }

    fn read(&self) -> &Any {
        self.inner_.get_data()
    }

    fn write(&mut self, val: Box<Any>) {
        match val.downcast::<Stock>() {
            Ok(val) => self.data_ = Some(val),
            Err(_) => panic!("Stock::write value should be Box<Warehouse>")
        }
    }


    fn set_access_info(&mut self, txn_info : Arc<TxnInfo> ) {
        self.inner_.set_access_info(txn_info);
    }

    fn get_access_info(&self) -> Arc<TxnInfo> {
        self.inner_.get_access_info()
    }
    fn get_name(&self) -> String {
        String::from("stock")
    }

    /* For 2 Phase Locking */
    fn read_lock(&self, tid: u32) -> bool {
        panic!("not implemented")
    }

    fn read_unlock(&self) {
        panic!("TODO");
    }

    fn write_lock(&self, tid: u32) -> bool {
        panic!("TODO");
    }

    fn write_unlock(&self, tid: u32)  {
        panic!("TODO");
    }
}


impl TableRef for Arc<Row<Warehouse, i32>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    ) 
        -> Box<dyn TRef> 
        {
            Box::new(
                WarehouseRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}

impl TableRef for Arc<Row<Customer, (i32, i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                CustomerRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}



impl TableRef for Arc<Row<District, (i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                DistrictRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}


impl TableRef for Arc<Row<NewOrder, (i32, i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                NewOrderRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None, 
                    ops_ : Operation::RWrite,
                })
        }
}


impl TableRef for Arc<Row<Order, (i32, i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                OrderRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}

impl TableRef for Arc<Row<OrderLine, (i32, i32, i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                OrderLineRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}


impl TableRef for Arc<Row<Item, i32>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                ItemRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}

impl TableRef for Arc<Row<Stock, (i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                StockRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None ,
                    ops_ : Operation::RWrite,
                })
        }
}

impl TableRef for Arc<Row<History,(i32, i32)>> {
    fn into_table_ref(
        self,
        bucket_idx: Option<usize>,
        //txn_info : Option<Arc<TxnInfo>>,
        table_ref : Option<Arc<Tables>>
    )
        -> Box<dyn TRef> 
        {
            Box::new(
                HistoryRef {
                    inner_ : self,
                    bucket_idx_: bucket_idx,
                    table_ref_: table_ref,
                    //txn_info_ : txn_info,
                    data_ : None,
                    ops_ : Operation::RWrite,
                })
        }
}

impl BucketDeleteRef for Arc<Row<NewOrder, (i32, i32, i32)>> {
    fn into_delete_table_ref(
        self,
        bucket_idx: usize,
        table_ref : Arc<Tables>
    )
        -> Box<dyn TRef>
        {
            Box::new(
                NewOrderRef {
                    inner_ : self,
                    bucket_idx_: Some(bucket_idx),
                    table_ref_: Some(table_ref),
                    data_ : None,
                    ops_ : Operation::Delete,
                })
        }

}

impl BucketPushRef for Arc<Row<Order, (i32, i32, i32)>> {
    fn into_push_table_ref(
        self,
        bucket_idx: usize,
        table_ref : Arc<Tables>
    ) 
        -> Box<dyn TRef>
        {
            Box::new(
                OrderRef {
                    inner_ : self,
                    bucket_idx_: Some(bucket_idx),
                    table_ref_: Some(table_ref),
                    data_ : None,
                    ops_: Operation::Push,
                })
        }
}


impl BucketPushRef for Arc<Row<NewOrder, (i32, i32, i32)>>
{
    fn into_push_table_ref(
        self,
        bucket_idx: usize,
        table_ref: Arc<Tables>
    )
        -> Box<dyn TRef>
        {
            Box::new(
                NewOrderRef {
                    inner_ : self,
                    bucket_idx_: Some(bucket_idx),
                    table_ref_: Some(table_ref),
                    data_ : None,
                    ops_ : Operation::Push,
                })
        }
}


impl BucketPushRef for Arc<Row<OrderLine, (i32, i32, i32, i32)>>
{
    fn into_push_table_ref(
        self,
        bucket_idx: usize,
        table_ref: Arc<Tables>
    )
        -> Box<dyn TRef>
        {
            Box::new(
                OrderLineRef {
                    inner_ : self,
                    bucket_idx_: Some(bucket_idx),
                    table_ref_: Some(table_ref),
                    data_ : None,
                    ops_ : Operation::Push,
                })
        }
}

impl BucketPushRef for Arc<Row<History, (i32, i32)>>
{
    fn into_push_table_ref(
        self,
        bucket_idx: usize,
        table_ref: Arc<Tables>
    )
        -> Box<dyn TRef>
        {
            Box::new(
                HistoryRef {
                    inner_ : self,
                    bucket_idx_: Some(bucket_idx),
                    table_ref_: Some(table_ref),
                    data_ : None,
                    ops_ : Operation::Push,
                })
        }
}





//impl BucketPushRef for Arc<Row<NewOrder, (i32, i32, i32)>> {
//    fn into_push_bucket_ref
//
//}
//

