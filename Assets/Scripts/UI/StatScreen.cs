using System;
using System.Collections;
using System.Collections.Generic;
using ObjectExtensions;
using UnityEngine;
using UnityEngine.InputSystem;

public class StatScreen : MonoBehaviour
{
    [Header( "Data" )]
    public GameObject hpPool;

    public GameObject hpSlider;

    public Entity observedEntity;

    private void Awake()
    {
        name = "StatScreen";
    }

    private void Start()
    {
        if ( hpPool.IsValid() ) hpPool.GetComponent< FlexibleUiBar >().observedEntity      = observedEntity;
        if ( hpSlider.IsValid() ) hpSlider.GetComponent< FlexibleUiText >().observedEntity = observedEntity;
    }
}