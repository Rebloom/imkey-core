package com.mk.imkeylibrary.device.model;


public class AppUpdateRequest extends CommonRequest {
    //SE唯一标识
    private String seid;
    private String instanceAid;//实例AID

    public String getSeid() {
        return seid;
    }

    public void setSeid(String seid) {
        this.seid = seid;
    }

    public String getInstanceAid() {
        return instanceAid;
    }

    public void setInstanceAid(String instanceAid) {
        this.instanceAid = instanceAid;
    }

    @Override
    public String toString() {
        return "AppUpdateRequest{" +
                "seid='" + seid + '\'' +
                ", instanceAid='" + instanceAid + '\'' +
                '}';
    }
}
